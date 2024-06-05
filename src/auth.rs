// Copyright 2024 bmc::labs GmbH. All rights reserved.

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const DEFAULT_VALIDITY_PERIOD_HOURS: i64 = 12;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String, // issuer
    exp: usize,  // expiration time - UTC timestamp in seconds
}

impl Claims {
    pub fn new(validity_period_days: Option<i64>) -> eyre::Result<Self> {
        let iss = "peripheral".to_string();
        let exp = Utc::now()
            .checked_add_signed(TimeDelta::hours(
                validity_period_days.unwrap_or(DEFAULT_VALIDITY_PERIOD_HOURS),
            ))
            .ok_or(eyre::eyre!("could not calculate expiration time"))?
            .timestamp() as usize;

        Ok(Self { iss, exp })
    }
}

pub fn init_secret() -> eyre::Result<String> {
    let Ok(secret) = std::env::var("SECRET") else {
        let err_msg = "SECRET not set in environment";

        tracing::error!(err_msg);
        eyre::bail!(err_msg);
    };

    Ok(secret)
}

pub fn encode_token(secret: &str) -> eyre::Result<String> {
    let token = encode(
        &Header::default(),
        &Claims::new(None)?,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    tracing::info!(?token, "generated token");

    Ok(token)
}

pub fn validate_token(secret: &str, token: &str) -> eyre::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    tracing::info!("token is valid");

    Ok(token_data.claims)
}
