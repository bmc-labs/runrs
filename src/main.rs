// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod crud;
mod runner;

use axum::{routing::get, Router};
use dotenv_codegen::dotenv;
use eyre::WrapErr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize color eyre and tracing
    setup()?;

    // initialize listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    // get handle to database
    let pool = atmosphere::Pool::connect(dotenv!("DATABASE_URL")).await?;

    // initialize router
    let app = app(pool).await?;

    // run app
    axum::serve(listener, app).await.wrap_err("server stopped")
}

async fn app(pool: atmosphere::Pool) -> eyre::Result<Router> {
    // set up app routing
    Ok(Router::new()
        .route("/", get(crud::read).post(crud::create))
        .route(
            "/:id",
            get(crud::read).put(crud::update).delete(crud::delete),
        )
        .with_state(pool))
}

/// Initializes backtracing and error handling capabilities. Sets up tracing and task monitoring
/// through tokio console.
fn setup() -> eyre::Result<()> {
    // set up eyre with colors
    const BT_ENVVAR: &str = "RUST_LIB_BACKTRACE";
    if std::env::var(BT_ENVVAR).is_err() {
        std::env::set_var(BT_ENVVAR, "1")
    }
    color_eyre::install()?;

    // set up format layer with filtering for tracing
    const LG_ENVVAR: &str = "RUST_LOG";
    if std::env::var(LG_ENVVAR).is_err() {
        std::env::set_var(LG_ENVVAR, "debug")
    }
    tracing_subscriber::fmt::init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{app, runner::Runner};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use pretty_assertions::assert_eq;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn create_delete() -> eyre::Result<()> {
        let pool = atmosphere::Pool::connect("sqlite::memory:").await.unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        let runner = Runner {
            id: 42,
            url: "https://gitlab.bmc-labs.com".to_owned(),
            token: "gltok-warblgarbl".to_owned(),
            description: "Knows the meaning of life".to_owned(),
            image: "alpine:latest".to_owned(),
            tag_list: "runnertest,wagarbl".to_owned(),
            run_untagged: false,
        };

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        Ok(())
    }

    #[tokio::test]
    async fn update() -> eyre::Result<()> {
        let pool = atmosphere::Pool::connect("sqlite::memory:").await.unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        let mut runner = Runner {
            id: 42,
            url: "https://gitlab.bmc-labs.com".to_owned(),
            token: "gltok-warblgarbl".to_owned(),
            description: "Knows the meaning of life".to_owned(),
            image: "alpine:latest".to_owned(),
            tag_list: "runnertest,wagarbl".to_owned(),
            run_untagged: false,
        };

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        runner.tag_list = "alpine,latest".to_string();

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(&format!("/{}", runner.id))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await?.to_bytes();
        let body: Runner = serde_json::from_slice(&body)?;

        assert_eq!(body, runner);

        Ok(())
    }
}
