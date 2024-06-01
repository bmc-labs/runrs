// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod runners;

use axum::routing::{get, post};
use axum::{middleware, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;
use crate::{error, model};

#[derive(OpenApi)]
#[openapi(
    paths(runners::create, runners::list, runners::read, runners::update, runners::delete,),
    components(
        schemas(
            error::Error,
            error::ErrorType,
            model::GitLabRunner
        )
    ),
    tags(
        (name = "runrs", description = "GitLab Runners Docker API")
    ),
    servers(
        (url = "http://0.0.0.0:3000/", description = "Local development server")
    )
)]
struct ApiDoc;

pub async fn app(secret: String, app_state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/runrs-api.json", ApiDoc::openapi()))
        .merge(
            Router::new()
                .route("/gitlab-runners", post(runners::create))
                .route("/gitlab-runners/list", get(runners::list))
                .route(
                    "/gitlab-runners/:id",
                    get(runners::read)
                        .put(runners::update)
                        .delete(runners::delete),
                )
                .layer(middleware::from_fn_with_state(secret, auth::authenticate)),
        )
        .with_state(app_state)
}

mod auth {
    use axum::extract::{Request, State};
    use axum::http::HeaderMap;
    use axum::middleware::Next;
    use axum::response::Response;

    // use jsonwebtoken::{decode, DecodingKey, Validation};
    // use crate::claims::Claims;
    // use crate::error::Error;

    pub async fn authenticate(
        headers: HeaderMap,
        State(_secret): State<String>,
        request: Request,
        next: Next,
    ) -> Response {
        tracing::debug!(?headers, "authenticating request");

        // let _ = decode::<Claims>(
        //     &token,
        //     &DecodingKey::from_secret(secret.as_ref()),
        //     &Validation::default(),
        // )
        // .map_err(Error::forbidden)
        // .into();

        next.run(request).await
    }
}
