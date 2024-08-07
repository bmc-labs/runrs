// Copyright 2024 bmc::labs GmbH. All rights reserved.

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        OpenApi,
    },
    Modify,
};

const DEFAULT_VALIDITY_PERIOD_HOURS: i64 = 12;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String, // issuer
    exp: usize,  // expiration time - UTC timestamp in seconds
}

impl Claims {
    pub fn new(validity_period_days: Option<i64>) -> miette::Result<Self> {
        let iss = "peripheral".to_string();
        let exp = Utc::now()
            .checked_add_signed(TimeDelta::hours(
                validity_period_days.unwrap_or(DEFAULT_VALIDITY_PERIOD_HOURS),
            ))
            .ok_or(miette::miette!("could not calculate expiration time"))?
            .timestamp() as usize;

        Ok(Self { iss, exp })
    }
}

pub fn init_secret() -> miette::Result<String> {
    let Ok(secret) = std::env::var("SECRET") else {
        let err_msg = "SECRET not set in environment";

        tracing::error!(err_msg);
        miette::bail!(err_msg);
    };

    Ok(secret)
}

pub fn encode_token(secret: &str) -> miette::Result<String> {
    let token = encode(
        &Header::default(),
        &Claims::new(None)?,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .into_diagnostic()?;

    tracing::info!(?token, "generated token");
    Ok(token)
}

pub fn validate_token(secret: &str, token: &str) -> miette::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .into_diagnostic()?;

    tracing::info!("token is valid");
    Ok(token_data.claims)
}

/// Authenticate middleware checks the request headers for a valid JWT token.
pub async fn authenticate(
    headers: HeaderMap,
    State(secret): State<String>,
    request: Request,
    next: Next,
) -> Response {
    tracing::debug!(?headers, "authenticating request");
    let err_response = (
        StatusCode::FORBIDDEN,
        Json("unable to authenticate request"),
    )
        .into_response();

    let Some(token) = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        tracing::warn!(?headers, "no token found in request headers");
        return err_response;
    };

    if validate_token(&secret, token).is_err() {
        tracing::warn!(?token, "unable to validate token");
        return err_response;
    };

    next.run(request).await
}

/// SecurityAddon is a modifier that adds a security scheme to the OpenAPI spec.
pub(super) struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut OpenApi) {
        openapi
            .components
            .as_mut()
            .expect("components not found - this is an error in runrs")
            .add_security_scheme(
                "api_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
    }
}
