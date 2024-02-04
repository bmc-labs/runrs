// Copyright 2024 bmc::labs GmbH. All rights reserved.

use crate::runner::Runner;
use atmosphere::{prelude::*, query::QueryError, Error};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[axum::debug_handler]
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

#[axum::debug_handler]
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
        Ok(resp) => {
            let Some(runner) = resp else {
                tracing::debug!(desc = "internal error", id = id,);
                return StatusCode::NOT_FOUND.into_response();
            };
            runner
        }
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

    (StatusCode::FOUND, Json(runner)).into_response()
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

#[axum::debug_handler]
#[tracing::instrument(skip(_pool))]
pub async fn update(State(_pool): State<Pool>, Json(_runner): Json<Runner>) -> Response {
    unimplemented!()
}

#[axum::debug_handler]
#[tracing::instrument(skip(pool))]
pub async fn delete(State(pool): State<Pool>, Path(id): Path<i32>) -> Response {
    tracing::info!("deleting runner with id {id} from database");
    tracing::debug!(id = ?id);

    let mut runner = match Runner::find(&id, &pool).await {
        Ok(resp) => {
            let Some(runner) = resp else {
                tracing::debug!(desc = "runner not found in database", id = id,);
                return StatusCode::NOT_FOUND.into_response();
            };
            runner
        }
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
