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

    // get handle to database
    let pool = atmosphere::Pool::connect(dotenv!("DATABASE_URL")).await?;

    // set up app routing
    let app = Router::new()
        .route("/", get(crud::read).post(crud::create))
        .route(
            "/:id",
            get(crud::read).put(crud::update).delete(crud::delete),
        )
        .with_state(pool);

    // run app
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.wrap_err("server stopped")
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
