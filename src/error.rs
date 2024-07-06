// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, PartialEq, Eq, Error, Diagnostic, Serialize, Deserialize, ToResponse, ToSchema)]
#[non_exhaustive]
pub enum ErrorType {
    #[error("connection failed")]
    ConnectionFailed,
    #[error("invalid argument")]
    InvalidArgument,
    #[error("runner already exists")]
    AlreadyExists,
    #[error("access forbidden")]
    Forbidden,
    #[error("request effects no changes")]
    Unchanged,
    #[error("runner not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("internal error")]
    InternalError,
    #[error("unimplemented")]
    Unimplemented,
    #[error("other")]
    Other,
}

#[derive(Debug, Error, Serialize, Deserialize, ToSchema)]
#[error("API Error: {msg}")]
pub struct Error {
    pub err_type: ErrorType,
    pub msg: String,
}

impl Error {
    pub fn new(err_type: ErrorType) -> Self {
        let msg = err_type.to_string();
        Self { err_type, msg }
    }

    pub fn with_description<T: Display>(mut self, desc: T) -> Self {
        self.msg = format!("{}: {}", self.msg, desc);
        tracing::error!("{}", self.msg);
        self
    }

    pub fn connection_failed<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::ConnectionFailed).with_description(desc)
    }

    pub fn invalid_argument<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::InvalidArgument).with_description(desc)
    }

    pub fn already_exists<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::AlreadyExists).with_description(desc)
    }

    pub fn forbidden<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::Forbidden).with_description(desc)
    }

    pub fn unchanged<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::Unchanged).with_description(desc)
    }

    pub fn not_found<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::NotFound).with_description(desc)
    }

    pub fn bad_request<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::BadRequest).with_description(desc)
    }

    pub fn internal_error<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::InternalError).with_description(desc)
    }

    pub fn unimplemented<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::Unimplemented).with_description(desc)
    }

    pub fn other<T: Display>(desc: T) -> Self {
        Self::new(ErrorType::Other).with_description(desc)
    }
}

impl From<sqlx::Error> for Error {
    fn from(sqlx_err: sqlx::Error) -> Self {
        use sqlx::error::Error as SqlxError;

        let mut msg = String::new();
        let err = match sqlx_err {
            SqlxError::RowNotFound => ErrorType::NotFound,
            SqlxError::Database(err) => {
                msg = err.to_string();
                ErrorType::InternalError
            }
            _ => ErrorType::Other,
        };

        Self::new(err).with_description(msg)
    }
}

impl From<atmosphere::Error> for Error {
    fn from(atm_err: atmosphere::Error) -> Self {
        use atmosphere::query::{QueryError, SqlError, ViolationError};

        let mut msg = String::new();
        let err = match atm_err {
            atmosphere::Error::Io(_) => ErrorType::ConnectionFailed,
            atmosphere::Error::Query(query_err) => match query_err {
                QueryError::Sql(SqlError::Other(err)) => return err.into(),
                QueryError::Sql(SqlError::DataException(err))
                | QueryError::Violation(ViolationError::ForeignKey(err)) => {
                    msg = err.to_string();
                    ErrorType::InvalidArgument
                }
                QueryError::Violation(ViolationError::Unique(err)) => {
                    msg = err.to_string();
                    ErrorType::AlreadyExists
                }
                QueryError::NotFound(err) => {
                    msg = err.to_string();
                    ErrorType::NotFound
                }
                QueryError::InternalError(err) => {
                    msg = err.to_string();
                    ErrorType::InternalError
                }
                QueryError::Other(err) => {
                    msg = err.to_string();
                    ErrorType::Other
                }
                _ => ErrorType::Other,
            },
            atmosphere::Error::Internal => ErrorType::InternalError,
            _ => ErrorType::Other,
        };

        Self::new(err).with_description(msg)
    }
}

impl From<Error> for Response {
    fn from(err: Error) -> Self {
        let status_code = match err.err_type {
            ErrorType::InvalidArgument | ErrorType::AlreadyExists | ErrorType::BadRequest => {
                StatusCode::BAD_REQUEST
            }
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Unimplemented => StatusCode::NOT_IMPLEMENTED,
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
            ErrorType::Unchanged => StatusCode::NO_CONTENT,
            ErrorType::ConnectionFailed | ErrorType::InternalError | ErrorType::Other => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status_code, Json(err)).into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::from(self)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn error_with_description() {
        let err = Error::new(ErrorType::Unimplemented);
        assert_eq!(&err.msg, "unimplemented");

        let desc = "this is a description";
        let err = err.with_description(desc);
        assert_eq!(err.msg, format!("unimplemented: {desc}"));
    }
}
