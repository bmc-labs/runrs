// Copyright 2024 bmc::labs GmbH. All rights reserved.

use crate::error::Error;
use crate::runner::Runner;

use atmosphere::prelude::*;
use atmosphere::query::QueryError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[utoipa::path(
    post,
    path = "/",
    params(
        Runner
    ),
    responses(
        (status = StatusCode::CREATED, description = "Created new Runner", body = Runner),
        (status = StatusCode::BAD_REQUEST, description = "Runner already exists", body = Error),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = Error)
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn create(State(pool): State<Pool>, Json(mut runner): Json<Runner>) -> Response {
    tracing::trace!(?runner, "writing runner with to database");

    if let Err(err) = runner.create(&pool).await {
        tracing::error!(?err, "database responded with error");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err:#?}"))).into_response();
    }

    tracing::debug!(desc = "runner written to database", id = runner.id);

    (StatusCode::CREATED, Json(runner.id)).into_response()
}

#[tracing::instrument(skip(pool))]
pub async fn read(id: Option<Path<i32>>, State(pool): State<Pool>) -> Response {
    tracing::info!("reading runner(s) from database");
    tracing::debug!(id = ?id);

    match id {
        Some(Path(id)) => read_one(id, pool).await,
        None => read_all(pool).await,
    }
}

#[tracing::instrument(skip(pool))]
pub async fn read_one(id: i32, pool: Pool) -> Response {
    tracing::info!("reading runner with id {id} from database");
    tracing::debug!(id = ?id);

    let runner = match Runner::find(&id, &pool).await {
        Ok(runner) => runner,
        Err(Error::Query(QueryError::NotFound(err))) => {
            tracing::debug!(
                desc = "runner not found",
                id = ?id,
                msg = ?err
            );
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            tracing::debug!(
                desc = "database responded with error",
                msg = ?err
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err:#?}"))).into_response();
        }
    };

    tracing::debug!(desc = "runner found in database", id = id);

    (StatusCode::OK, Json(runner)).into_response()
}

#[tracing::instrument(skip(pool))]
pub async fn read_all(pool: Pool) -> Response {
    tracing::info!("reading all runners from database");

    let runners = match Runner::find_all(&pool).await {
        Ok(runners) => runners,
        Err(err) => {
            tracing::debug!(
                desc = "database responded with error",
                msg = ?err
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")).into_response();
        }
    };

    tracing::debug!(
        desc = "runners returned from database",
        runners = ?runners
    );

    (StatusCode::OK, Json(runners)).into_response()
}

#[tracing::instrument(skip(pool))]
pub async fn update(State(pool): State<Pool>, Json(mut runner): Json<Runner>) -> Response {
    tracing::info!("updating runner with id {} in database", runner.id);
    tracing::debug!(runner = ?runner);

    if read_one(runner.id, pool.clone()).await.status() != StatusCode::OK {
        return create(State(pool), Json(runner)).await;
    }

    if let Err(err) = runner.save(&pool).await {
        tracing::debug!(
            desc = "database responded with error",
            msg = ?err
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err:#?}"))).into_response();
    }

    tracing::debug!(desc = "runner updated in database", id = runner.id);

    (StatusCode::OK, Json(runner.id)).into_response()
}

#[tracing::instrument(skip(pool))]
pub async fn delete(State(pool): State<Pool>, Path(id): Path<i32>) -> Response {
    tracing::info!("deleting runner with id {id} from database");
    tracing::debug!(id = ?id);

    let mut runner = match Runner::find(&id, &pool).await {
        Ok(runner) => runner,
        Err(Error::Query(QueryError::NotFound(err))) => {
            tracing::debug!(
                desc = "runner not found",
                id = ?id,
                msg = ?err
            );
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            tracing::debug!(
                desc = "database responded with error",
                msg = ?err
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err:#?}"))).into_response();
        }
    };

    if let Err(err) = runner.delete(&pool).await {
        tracing::debug!(
            desc = "database responded with error",
            msg = ?err
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("{err:#?}"))).into_response();
    }

    tracing::debug!(desc = "runner written to database", id = runner.id);

    (StatusCode::OK, Json(runner.id)).into_response()
}

#[cfg(test)]
mod tests {
    use crate::rest::app;
    use crate::runner::Runner;
    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use http_body_util::BodyExt; // for `collect`
    use pretty_assertions::assert_eq;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[sqlx::test]
    #[tracing_test::traced_test]
    async fn create_delete(pool: atmosphere::Pool) -> eyre::Result<()> {
        // Set up a testing `Runner` and a reusable `Request`
        let runner = Runner::for_testing();
        let request = Request::builder()
            .method(http::Method::GET)
            .uri(&format!("/{}", runner.id))
            .body(String::new())?;

        // Assert that the `Runner` is not in the database using the API call
        let response = app(pool.clone()).await?.oneshot(request.clone()).await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Insert the `Runner` in the database using the API call and assert `CREATED`
        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::CREATED);

        // Assert that the `Runner` is in the database using the API call
        let response = app(pool.clone()).await?.oneshot(request.clone()).await?;
        assert_eq!(response.status(), StatusCode::OK);

        // Delete the `Runner` from the database using the API call and assert `OK`
        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);

        // Assert that the `Runner` is not in the database using the API call
        let response = app(pool.clone()).await?.oneshot(request).await?;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        Ok(())
    }

    #[sqlx::test]
    #[tracing_test::traced_test]
    async fn update(pool: atmosphere::Pool) -> eyre::Result<()> {
        let mut runner = Runner::for_testing();

        // Create the runner in the database using the API call and assert `CREATED`
        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        // Change field in the `Runner`
        runner.tag_list = "alpine,latest".to_string();

        // Update the `Runner` in the database using the API call and assert `OK`
        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(&format!("/{}", runner.id))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(&runner)?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        // Retrieve the `Runner` from the database using the API call and assert `OK`
        let response = app(pool.clone())
            .await?
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(&format!("/{}", runner.id))
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        // Assert that the `Runner` from the response body is the same as the updated `Runner`
        let body = response.into_body().collect().await?.to_bytes();
        let body: Runner = serde_json::from_slice(&body)?;

        assert_eq!(body, runner);

        Ok(())
    }
}
