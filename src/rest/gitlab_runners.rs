// Append or overwrite environment variables. Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{Create as _, Delete as _, Read as _, Update as _};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    error::Error,
    model::{GitLabRunner, GitLabRunnerConfig},
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/gitlab-runners",
    request_body(
        content = GitLabRunner, description = "GitLabRunner to update", content_type = "application/json"
    ),
    responses(
        (status = StatusCode::CREATED, description = "Created new GitLab Runner", body = GitLabRunner),
        (status = StatusCode::BAD_REQUEST, description = "GitLab Runner already exists", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn create(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Json(mut runner): Json<GitLabRunner>,
) -> Response {
    tracing::debug!(?runner, "creating runner in database");

    if let Err(err) = runner.create(&pool).await {
        tracing::error!(?err, "database responded with error");
        return Error::from(err).into();
    }
    tracing::debug!(?runner, "runner written to database");

    if let Err(err) = GitLabRunnerConfig::write(&pool, &config_path).await {
        tracing::error!(?err, "Error in writing config.toml");
        return Error::internal_error("unable to write to runner config").into();
    }
    tracing::debug!("GitLabRunnerConfig written to disk");

    (StatusCode::CREATED, Json(runner)).into_response()
}

#[utoipa::path(
    get,
    path = "/gitlab-runners/list",
    responses(
        (status = StatusCode::OK, description = "Read all GitLabRunners", body = GitLabRunner),
        (status = StatusCode::NOT_FOUND, description = "GitLabRunner not found", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn list(State(AppState { pool, .. }): State<AppState>) -> Response {
    tracing::debug!("reading all runners from database");

    let runners = match GitLabRunner::find_all(&pool).await {
        Ok(runners) => runners,
        Err(err) => {
            tracing::debug!(?err, "database responded with error");
            return Error::from(err).into();
        }
    };

    tracing::debug!(?runners, "runners returned from database");

    (StatusCode::OK, Json(runners)).into_response()
}

#[utoipa::path(
    get,
    path = "/gitlab-runners/{id}",
    params(
        ("id" = String, Path, description = "GitLabRunner ID")
    ),
    responses(
        (status = StatusCode::OK, description = "Read all GitLabRunners", body = GitLabRunner),
        (status = StatusCode::NOT_FOUND, description = "GitLabRunner not found", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    tracing::info!("reading runner with id {id} from database");
    tracing::debug!(id = ?id);

    let runner = match GitLabRunner::find(&id, &pool).await {
        Ok(runner) => runner,
        Err(err) => {
            tracing::debug!(?err, "database responded with error");
            return Error::from(err).into();
        }
    };

    tracing::debug!(desc = "runner found in database", id = id);

    (StatusCode::OK, Json(runner)).into_response()
}

#[utoipa::path(
    put,
    path = "/gitlab-runners/{id}",
    params(
        ("id" = String, Path, description = "GitLabRunner ID")
    ),
    request_body(
        content = GitLabRunner, description = "GitLabRunner to update", content_type = "application/json"
    ),
    responses(
        (status = StatusCode::OK, description = "Updated GitLabRunner", body = GitLabRunner),
        (status = StatusCode::NO_CONTENT, description = "GitLabRunner already up-to-date"),
        (status = StatusCode::NOT_FOUND, description = "GitLabRunner not found", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn update(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Path(id): Path<String>,
    Json(updated_runner): Json<GitLabRunner>,
) -> Response {
    tracing::debug!(?id, ?updated_runner, "updating runner");

    let mut runner = match GitLabRunner::find(&id, &pool).await {
        Ok(runner) => runner,
        Err(err) => {
            tracing::error!(?err, "database responded with error");
            return Error::from(err).into();
        }
    };

    if let Err(err) = runner.update(updated_runner) {
        tracing::error!(?err, "error updating runner");
        return Error::invalid_argument(err).into();
    }

    if let Err(err) = runner.save(&pool).await {
        tracing::error!(?err, "database responded with error");
        return Error::from(err).into();
    }

    tracing::debug!(?id, ?runner, "runner updated");

    if let Err(err) = GitLabRunnerConfig::write(&pool, &config_path).await {
        tracing::error!(?err, "Error in writing config.toml");
        return Error::internal_error("unable to write to runner config").into();
    }
    tracing::debug!("GitLabRunnerConfig written to disk");

    (StatusCode::OK, Json(runner)).into_response()
}

#[utoipa::path(
    delete,
    path = "/gitlab-runners/{id}",
    params(
        ("id" = String, Path, description = "GitLabRunner ID")
    ),
    responses(
        (status = StatusCode::OK, description = "Deleted GitLabRunner", body = GitLabRunner),
        (status = StatusCode::NOT_FOUND, description = "GitLabRunner not found", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn delete(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    tracing::debug!(?id, "deleting runner with id");

    let mut runner = match GitLabRunner::find(&id, &pool).await {
        Ok(runner) => runner,
        Err(err) => {
            tracing::debug!(?err, "database responded with error");
            return Error::from(err).into();
        }
    };

    if let Err(err) = runner.delete(&pool).await {
        tracing::debug!(?err, "database responded with error");
        return Error::from(err).into();
    }
    tracing::debug!(?runner, "runner deleted");

    if let Err(err) = GitLabRunnerConfig::write(&pool, &config_path).await {
        tracing::error!(?err, "Error in writing config.toml");
        return Error::internal_error("unable to write to runner config").into();
    }
    tracing::debug!("GitLabRunnerConfig written to disk");

    (StatusCode::OK, Json(runner)).into_response()
}

#[cfg(test)]
mod tests {
    use atmosphere::{Read as _, Update as _};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    use crate::{auth, model::GitLabRunner, rest::app, state::AppState};

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    #[tracing_test::traced_test]
    async fn create_delete(pool: atmosphere::Pool) -> eyre::Result<()> {
        let secret = "test-secret".to_string();
        let app_state = AppState::for_testing(pool);

        let token = auth::encode_token(&secret)?;

        let runner = GitLabRunner::for_testing();
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/gitlab-runners/{}", runner.id()))
            .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
            .body(String::new())?;

        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(request.clone())
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/gitlab-runners")
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(request.clone())
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(&format!("/gitlab-runners/{}", runner.id()))
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(request)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        std::fs::remove_file(&app_state.config_path)?;

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    #[tracing_test::traced_test]
    async fn update(pool: atmosphere::Pool) -> eyre::Result<()> {
        let secret = "test-secret".to_string();
        let app_state = AppState::for_testing(pool);

        let token = auth::encode_token(&secret)?;

        let mut runner = GitLabRunner::for_testing();
        runner.save(&app_state.pool).await?;

        runner.set_url("https://gitlab.bmc-labs.com");
        let response = app(secret.clone(), app_state.clone())
            .await
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(&format!("/gitlab-runners/{}", runner.id()))
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let runner_from_db = GitLabRunner::find(runner.id(), &app_state.pool).await?;
        assert_eq!(runner_from_db, runner);

        std::fs::remove_file(&app_state.config_path)?;

        Ok(())
    }
}
