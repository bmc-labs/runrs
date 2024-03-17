// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod error;
mod rest;
mod runner;

use dotenv_codegen::dotenv;
use miette::WrapErr;

#[tokio::main]
async fn main() -> miette::Result<()> {
    // set envvar defaults and init tracing
    tracing_init()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    let pool = atmosphere::Pool::connect(dotenv!("DATABASE_URL")).await?;

    // initialize router and run app
    let app = rest::app(pool).await?;
    axum::serve(listener, app).await.wrap_err("server stopped")
}

/// Initializes backtracing and error handling capabilities. Sets up tracing and task monitoring
/// through tokio console.
fn tracing_init() -> miette::Result<()> {
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
