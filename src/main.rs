// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod app;
mod auth;
mod error;
mod handlers;
mod models;

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

    let app_state = app::AppState::init().await?;

    // initialize router and run app
    let router = app::router(secret, app_state).await;

    if let Err(err) = axum::serve(listener, router).await {
        tracing::error!(%err, "Server stopped");
        eyre::bail!(err);
    }

    Ok(())
}

mod logging {
    use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

    /// Initializes backtracing and error handling capabilities.
    pub fn init() -> eyre::Result<()> {
        // Logs in prod environments are often expensive,
        // incurring per-MB costs in some cases (e.g. AWS).
        // We therefore default to ERROR level for everything
        // except runrs itself, which defaults to WARN.
        let filter =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::try_new("error,runrs=warn")?);

        let subscriber = tracing_subscriber::fmt().with_env_filter(filter);

        match std::env::var("LOG_FMT") {
            Ok(fmt) if fmt == "json" => subscriber.json().finish().init(),
            _ => subscriber.finish().init(),
        }

        Ok(())
    }
}
