// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod app_state;
mod config;
mod error;
mod model;
mod rest;

// Embed database migrations in the binary
pub(crate) static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // set envvar defaults and init tracing
    monitoring::init()?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("REST API on http://{}", listener.local_addr()?);
    tracing::info!("API docs on http://{}/api-docs", listener.local_addr()?);

    let app_state = app_state::AppState::init().await?;

    // initialize router and run app
    let app = rest::app(app_state).await?;

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(%err, "Server stopped");
        eyre::bail!(err);
    }

    Ok(())
}

mod monitoring {
    use std::str::FromStr;

    use tracing::Level;
    use tracing_subscriber::filter::Targets;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    /// Initializes backtracing and error handling capabilities.
    pub fn init() -> eyre::Result<()> {
        const BT_ENVVAR: &str = "RUST_LIB_BACKTRACE";
        if std::env::var(BT_ENVVAR).is_err() {
            std::env::set_var(BT_ENVVAR, "1")
        }

        // set up format layer with filtering for tracing
        const LG_ENVVAR: &str = "RUST_LOG";
        if std::env::var(LG_ENVVAR).is_err() {
            std::env::set_var(LG_ENVVAR, "error,runrs=debug")
        }

        let filter = Targets::from_str(
            std::env::var("RUST_LOG")
                .as_deref()
                .unwrap_or("error,runrs=debug"),
        )?;

        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            // TODO(flrn): turn on JSON once we start logging to a service
            // .json()
            .finish()
            .with(filter)
            .init();

        Ok(())
    }
}
