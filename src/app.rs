// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fs::File, path::PathBuf, time::Duration};

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use miette::IntoDiagnostic;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    auth::{authenticate, SecurityAddon},
    error,
    handlers::gitlab_runners,
    models,
};

pub static DEFAULT_DATABASE_URL: &str = "/etc/runrs/database.sqlite";
pub static DEFAULT_CONFIG_PATH: &str = "/etc/gitlab-runner/config.toml";
pub static REQUEST_TIMEOUT_SECS: u64 = 15;

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
        .layer((
            // outer tracing layer
            TraceLayer::new_for_http(),
            // set timeout for all requests
            TimeoutLayer::new(Duration::from_secs(REQUEST_TIMEOUT_SECS)),
        ))
        .with_state(app_state)
}

/// Holds the state for the API router
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: atmosphere::Pool,
    pub config_path: PathBuf,
}

impl AppState {
    pub async fn init() -> miette::Result<Self> {
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

async fn init_database() -> miette::Result<atmosphere::Pool> {
    let database_url = std::env::var("DATABASE_URL").map_or_else(
        |_| {
            tracing::warn!("DATABASE_URL not set, using default URL '{DEFAULT_DATABASE_URL}'");
            PathBuf::from(DEFAULT_DATABASE_URL)
        },
        PathBuf::from,
    );

    if !database_url.exists() {
        tracing::warn!(?database_url, "Database file not found, creating it");

        if let Some(base_path) = database_url.parent() {
            if !base_path.exists() {
                tracing::warn!(?base_path, "Database directory not found, creating it");
                std::fs::create_dir_all(base_path).into_diagnostic()?;
            }
        }

        File::create(&database_url).into_diagnostic()?;
    }

    let pool = match atmosphere::Pool::connect(
        database_url
            .to_str()
            .ok_or_else(|| miette::miette!("Invalid database URL"))?,
    )
    .await
    {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!(%err, "Failed to connect to database");
            miette::bail!(err);
        }
    };

    if let Err(err) = crate::MIGRATOR.run(&pool).await {
        tracing::error!(%err, "Failed to run migrations");
        miette::bail!(err);
    }

    Ok(pool)
}

fn init_config_path() -> miette::Result<PathBuf> {
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
                std::fs::create_dir_all(base_path).into_diagnostic()?;
            }
        }

        File::create(&config_path).into_diagnostic()?;
    }

    Ok(config_path)
}
