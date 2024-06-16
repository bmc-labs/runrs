// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod gitlab_runners;

use auth::SecurityAddon;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{error, models, state::AppState};

#[derive(OpenApi)]
#[openapi(
    paths(
        gitlab_runners::create,
        gitlab_runners::list,
        gitlab_runners::read,
        gitlab_runners::update,
        gitlab_runners::delete,
    ),
    components(
        schemas(
            error::Error,
            error::ErrorType,
            models::GitLabRunner,
        )
    ),
    tags(
        (name = "runrs", description = "GitLab Runners Docker API")
    ),
    servers(
        (url = "http://0.0.0.0:3000/", description = "Local development server")
    ),
    security(
        ("api_token" = [])
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

pub async fn app(secret: String, app_state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/runrs-api.json", ApiDoc::openapi()))
        .merge(
            Router::new()
                .route("/gitlab-runners", post(gitlab_runners::create))
                .route("/gitlab-runners/list", get(gitlab_runners::list))
                .route(
                    "/gitlab-runners/:id",
                    get(gitlab_runners::read)
                        .put(gitlab_runners::update)
                        .delete(gitlab_runners::delete),
                )
                .layer(middleware::from_fn_with_state(secret, auth::authenticate)),
        )
        .with_state(app_state)
}

mod auth {
    use axum::{
        extract::{Request, State},
        http::{header, HeaderMap},
        middleware::Next,
        response::Response,
    };
    use utoipa::{
        openapi::{
            security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
            OpenApi,
        },
        Modify,
    };

    use crate::{auth::validate_token, error::Error};

    /// Authenticate middleware checks the request headers for a valid JWT token.
    pub async fn authenticate(
        headers: HeaderMap,
        State(secret): State<String>,
        request: Request,
        next: Next,
    ) -> Response {
        tracing::debug!(?headers, "authenticating request");

        let err = Error::forbidden("unable to authenticate request");

        let Some(token) = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
        else {
            tracing::warn!(?headers, "no token found in request headers");
            return err.into();
        };

        let Ok(_) = validate_token(&secret, token) else {
            tracing::warn!(?token, "unable to decode token");
            return err.into();
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
}
