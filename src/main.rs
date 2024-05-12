// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod error;
mod model;
mod monitoring;
mod rest;

pub(crate) static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

mod interfaces {
    pub mod gitlab;
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // set envvar defaults and init tracing
    monitoring::init()?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("REST API on http://{}", listener.local_addr()?);
    tracing::info!("API docs on http://{}/api-docs", listener.local_addr()?);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        tracing::warn!("DATABASE_URL not set, using in-memory database");
        "sqlite::memory:".to_string()
    });

    let pool = match atmosphere::Pool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!(%err, "Failed to connect to database");
            eyre::bail!(err);
        }
    };

    if let Err(err) = MIGRATOR.run(&pool).await {
        tracing::error!(%err, "Failed to run migrations");
        eyre::bail!(err);
    }

    // initialize router and run app
    let app = rest::app(pool).await?;

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!(%err, "Server stopped");
        eyre::bail!(err);
    }

    Ok(())
}
