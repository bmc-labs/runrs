// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod handlers;

use crate::runner;
use axum::routing::get;
use axum::Router;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::create, handlers::read, handlers::update, handlers::delete,),
    components(schemas(runner::Runner)),
    tags(
        (name = "runrs", description = "GitLab Runners Docker API")
    )
)]
struct ApiDoc;

pub async fn app(pool: atmosphere::Pool) -> eyre::Result<Router> {
    // set up app routing
    Ok(Router::new()
        .route("/runners/", get(handlers::read).post(handlers::create))
        .route(
            "/runners/:id",
            get(handlers::read)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .with_state(pool))
}
