// Copyright 2024 bmc::labs GmbH. All rights reserved.

use crate::runner::Runner;
use atmosphere::{prelude::*, query::QueryError, Error};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[tracing::instrument(skip(pool))]
pub async fn create(State(pool): State<Pool>, Json(mut runner): Json<Runner>) -> Response {
    tracing::info!("writing runner with id {} to database", runner.id);
    tracing::debug!(runner = ?runner);

    if let Err(err) = runner.create(&pool).await {
        tracing::debug!(
            desc = "database responded with error",
            msg = ?err
        );
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
