// Append or overwrite environment variables. Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{Create, Delete, Read, Update};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Result},
    Json,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::Error,
    models::{GitLabRunner, GitLabRunnerConfig},
};

#[utoipa::path(
    post,
    path = "/gitlab-runners",
    request_body(
        content = GitLabRunner, description = "GitLabRunner to create", content_type = "application/json"
    ),
    responses(
        (status = StatusCode::CREATED, description = "Created new GitLab Runner", body = GitLabRunner),
        (status = StatusCode::BAD_REQUEST, description = "GitLab Runner already exists", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool, config_path, runner))]
pub async fn create(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Json(mut runner): Json<GitLabRunner>,
) -> Result<Response> {
    tracing::debug!(?runner, "creating runner in database");

    runner.create(&pool).await.map_err(Error::from)?;
    tracing::debug!("runner written to database");

    GitLabRunnerConfig::write(&pool, &config_path)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runners config written to disk");

    Ok((StatusCode::CREATED, Json(runner)).into_response())
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
pub async fn list(State(AppState { pool, .. }): State<AppState>) -> Result<Response> {
    tracing::debug!("reading all runners from database");

    let runners = GitLabRunner::read_all(&pool).await.map_err(Error::from)?;
    tracing::debug!(?runners, "runners returned from database");

    Ok((StatusCode::OK, Json(runners)).into_response())
}

#[utoipa::path(
    get,
    path = "/gitlab-runners/{uuid}",
    params(
        ("uuid" = Uuid, Path, description = "GitLabRunner UUID")
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
    Path(uuid): Path<Uuid>,
) -> Result<Response> {
    tracing::debug!("reading runner from database");

    let runner = GitLabRunner::read(&pool, &uuid)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runner found in database");

    Ok((StatusCode::OK, Json(runner)).into_response())
}

#[utoipa::path(
    put,
    path = "/gitlab-runners/{uuid}",
    params(
        ("uuid" = Uuid, Path, description = "GitLab Runner UUID")
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
#[tracing::instrument(skip(pool, config_path, updated_runner))]
pub async fn update(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(mut updated_runner): Json<GitLabRunner>,
) -> Result<Response> {
    tracing::debug!(?updated_runner, "updating runner");

    let runner = GitLabRunner::read(&pool, &uuid)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runner found in database");

    if !updated_runner.compatible_with(&runner) {
        return Err(Error::invalid_argument("incompatible runner").into());
    }

    updated_runner.update(&pool).await.map_err(Error::from)?;
    tracing::debug!("runner updated");

    GitLabRunnerConfig::write(&pool, &config_path)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runners config written to disk");

    Ok((StatusCode::OK, Json(updated_runner)).into_response())
}

#[utoipa::path(
    delete,
    path = "/gitlab-runners/{uuid}",
    params(
        ("uuid" = Uuid, Path, description = "GitLabRunner UUID")
    ),
    responses(
        (status = StatusCode::OK, description = "Deleted GitLabRunner", body = GitLabRunner),
        (status = StatusCode::NOT_FOUND, description = "GitLabRunner not found", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool, config_path))]
pub async fn delete(
    State(AppState {
        pool, config_path, ..
    }): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Response> {
    tracing::debug!("deleting runner");

    let mut runner = GitLabRunner::read(&pool, &uuid)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runner found in database");

    runner.delete(&pool).await.map_err(Error::from)?;
    tracing::debug!("runner deleted");

    GitLabRunnerConfig::write(&pool, &config_path)
        .await
        .map_err(Error::from)?;
    tracing::debug!("runners config written to disk");

    Ok((StatusCode::OK, Json(runner)).into_response())
}

#[cfg(test)]
mod tests {
    use atmosphere::{Create, Read};
    use axum::{
        body::{to_bytes, Body},
        http::{self, Request, StatusCode},
    };
    use pretty_assertions::assert_eq;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    use crate::{
        app::{router, AppState},
        auth,
        models::GitLabRunner,
    };

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    #[tracing_test::traced_test]
    async fn create_delete(pool: atmosphere::Pool) -> Result<()> {
        let secret = "test-secret".to_string();
        let app_state = AppState::for_testing(pool);

        let token = auth::encode_token(&secret)?;

        let runner = GitLabRunner::for_testing();
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/gitlab-runners/{}", runner.uuid()))
            .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
            .body(String::new())?;

        let response = router(secret.clone(), app_state.clone())
            .await
            .oneshot(request.clone())
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = router(secret.clone(), app_state.clone())
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

        let response = router(secret.clone(), app_state.clone())
            .await
            .oneshot(request.clone())
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = router(secret.clone(), app_state.clone())
            .await
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(&format!("/gitlab-runners/{}", runner.uuid()))
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let response = router(secret.clone(), app_state.clone())
            .await
            .oneshot(request)
            .await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        std::fs::remove_file(&app_state.config_path)?;

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    #[tracing_test::traced_test]
    async fn update(pool: atmosphere::Pool) -> Result<()> {
        let secret = "test-secret".to_string();
        let app_state = AppState::for_testing(pool);

        let token = auth::encode_token(&secret)?;

        let mut runner = GitLabRunner::for_testing();
        runner.create(&app_state.pool).await?;

        runner.set_url("https://gitlab.bmc-labs.com");
        let response = router(secret.clone(), app_state.clone())
            .await
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(&format!("/gitlab-runners/{}", runner.uuid()))
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        let runner_from_response: GitLabRunner =
            serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await?)?;
        assert_eq!(runner_from_response, runner);

        let runner_from_db = GitLabRunner::read(&app_state.pool, runner.uuid()).await?;
        assert_eq!(runner_from_db, runner);

        std::fs::remove_file(&app_state.config_path)?;

        Ok(())
    }
}
