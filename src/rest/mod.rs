// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod runners;

use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{error, model};

#[derive(OpenApi)]
#[openapi(
    paths(runners::create, runners::list, runners::read, runners::update, runners::delete,),
    components(
        schemas(
            error::Error,
            error::ErrorType,
            model::Runner
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

pub async fn app(pool: atmosphere::Pool) -> eyre::Result<Router> {
    Ok(Router::new()
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(
            Router::new()
                .route("/runners", post(runners::create))
                .route("/runners/list", get(runners::list))
                .route(
                    "/runners/:id",
                    get(runners::read)
                        .put(runners::update)
                        .delete(runners::delete),
                ),
        )
        .with_state(pool))
}
