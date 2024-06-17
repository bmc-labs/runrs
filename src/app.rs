// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fs::File, path::PathBuf};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use eyre::WrapErr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    auth::{authenticate, SecurityAddon},
    config::{DEFAULT_CONFIG_PATH, DEFAULT_DATABASE_URL},
    error,
    handlers::gitlab_runners,
    models,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        gitlab_runners::create,
        gitlab_runners::list,
        gitlab_runners::read,
        gitlab_runners::update,
        gitlab_runners::delete,
    ),
    components(
        schemas(
            error::Error,
            error::ErrorType,
            models::GitLabRunner,
        )
    ),
    tags(
        (name = "runrs", description = "GitLab Runners Docker API")
    ),
    servers(
        (url = "http://0.0.0.0:3000/", description = "Local development server")
    ),
    security(
        ("api_token" = [])
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

/// Initializes the API router
pub async fn router(secret: String, app_state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/runrs-api.json", ApiDoc::openapi()))
        .merge(
            Router::new()
                .route("/gitlab-runners", post(gitlab_runners::create))
                .route("/gitlab-runners/list", get(gitlab_runners::list))
                .route(
                    "/gitlab-runners/:id",
                    get(gitlab_runners::read)
                        .put(gitlab_runners::update)
                        .delete(gitlab_runners::delete),
                )
                .layer(middleware::from_fn_with_state(secret, authenticate)),
        )
        .with_state(app_state)
}

/// Holds the state for the API router
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

#[cfg(test)]
impl AppState {
    pub fn for_testing(pool: atmosphere::Pool) -> Self {
        let config_path = PathBuf::from(format!(
            "/tmp/gitlab-runner-config-{}.toml",
            uuid::Uuid::new_v4()
        ));

        Self { pool, config_path }
    }
}

async fn init_database() -> eyre::Result<atmosphere::Pool> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        tracing::warn!("DATABASE_URL not set, using in-memory database");
        DEFAULT_DATABASE_URL.to_string()
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
            tracing::warn!("CONFIG_PATH not set, using default path '{DEFAULT_CONFIG_PATH}'");
            PathBuf::from(DEFAULT_CONFIG_PATH)
        },
        PathBuf::from,
    );

    if !config_path.exists() {
        tracing::warn!(
            ?config_path,
            "Config file or path not found, creating it with empty file"
        );

        if let Some(base_path) = config_path.parent() {
            if !base_path.exists() {
                tracing::warn!(?base_path, "Config directory not found, creating it");
                std::fs::create_dir_all(base_path).wrap_err("could not create config directory")?;
            }
        }

        File::create(&config_path).wrap_err("could not create config file")?;
    }

    Ok(config_path)
}
