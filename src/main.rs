// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod error;
mod model;
mod rest;

use dotenv_codegen::dotenv;
use eyre::WrapErr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // set envvar defaults and init tracing
    tracing_init()?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("REST API on http://{}", listener.local_addr()?);
    tracing::info!("API docs on http://{}/api-docs", listener.local_addr()?);

    let pool = atmosphere::Pool::connect(dotenv!("DATABASE_URL")).await?;

    // initialize router and run app
    let app = rest::app(pool).await?;
    axum::serve(listener, app).await.wrap_err("server stopped")
}

/// Initializes backtracing and error handling capabilities.
fn tracing_init() -> eyre::Result<()> {
    const BT_ENVVAR: &str = "RUST_LIB_BACKTRACE";
    if std::env::var(BT_ENVVAR).is_err() {
        std::env::set_var(BT_ENVVAR, "1")
    }

    // set up format layer with filtering for tracing
    const LG_ENVVAR: &str = "RUST_LOG";
    if std::env::var(LG_ENVVAR).is_err() {
        std::env::set_var(LG_ENVVAR, "debug")
    }
    tracing_subscriber::fmt::init();

    Ok(())
}
