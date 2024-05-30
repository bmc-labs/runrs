// Copyright 2024 bmc::labs GmbH. All rights reserved.

use axum::extract::Request;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {}

pub async fn authenticate(headers: HeaderMap, request: Request, next: Next) -> Response {
    tracing::debug!(?headers, "authenticating request");

    next.run(request).await
}
