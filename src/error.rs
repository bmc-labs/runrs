// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::backtrace::Backtrace;

use miette::Diagnostic;
use thiserror::Error;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Diagnostic, Error)]
#[non_exhaustive]
enum ErrorType {
    #[error("runner not found")]
    NotFound,
    #[error("runner already exists")]
    AlreadyExists,
    #[error("database responded with error")]
    Database,
    #[error("runner creation failed")]
    CreationFailed,
    #[error("runner could not be updated")]
    UpdateFailed,
    #[error("runner could not be deleted")]
    DeletionFailed,
    #[error("runner command exited with error")]
    RunnerCommand,
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Error, ToSchema, IntoParams)]
#[error("API Error: {msg}")]
pub struct Error {
    err: ErrorType,
    msg: String,
}
