// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod auth;
mod config;
mod error;
mod handlers;
mod logging;
mod models;
mod state;

// Embed database migrations in the binary
pub(crate) static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // set envvar defaults and init tracing
    logging::init()?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("REST API on http://{}", listener.local_addr()?);
    tracing::info!("API docs on http://{}/api-docs", listener.local_addr()?);

    let secret = auth::init_secret()?;
    let _ = auth::encode_token(&secret)?;

    let app_state = state::AppState::init().await?;

    // initialize router and run app
    let app = handlers::app(secret, app_state).await;

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(%err, "Server stopped");
        eyre::bail!(err);
    }

    Ok(())
}
