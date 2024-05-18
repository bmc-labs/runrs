// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::fs::File;
use std::path::PathBuf;

use eyre::WrapErr;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: atmosphere::Pool,
    pub config_path: PathBuf,
}

impl AppState {
    pub async fn init() -> eyre::Result<Self> {
        Ok(Self {
            pool: init_database().await?,
            config_path: init_config_path()?,
        })
    }
}

async fn init_database() -> eyre::Result<atmosphere::Pool> {
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

    if let Err(err) = crate::MIGRATOR.run(&pool).await {
        tracing::error!(%err, "Failed to run migrations");
        eyre::bail!(err);
    }

    Ok(pool)
}

fn init_config_path() -> eyre::Result<PathBuf> {
    let config_path = std::env::var("CONFIG_PATH").map_or_else(
        |_| {
            let default_path = "/etc/gitlab-runner/config.toml";
            tracing::warn!("CONFIG_PATH not set, using default path '{default_path}'");

            PathBuf::from(default_path)
        },
        PathBuf::from,
    );

    if !config_path.exists() {
        tracing::warn!(?config_path, "Config file not found, creating empty file");
        File::create(&config_path).wrap_err("could not create config file")?;
    }

    Ok(config_path)
}
