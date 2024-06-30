// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::path::PathBuf;

use atmosphere::Read;
use glrcfg::{Config, Runner};

use super::GitLabRunner;
use crate::error::Error;

#[derive(Debug)]
pub struct GitLabRunnerConfig(Config);

impl GitLabRunnerConfig {
    pub async fn compile(pool: &atmosphere::Pool) -> Result<Self, Error> {
        let runners = GitLabRunner::read_all(pool)
            .await?
            .into_iter()
            .map(Runner::from)
            .collect();

        let config = Config::builder().with_runners(runners).finish();

        Ok(Self(config))
    }

    pub async fn write(pool: &atmosphere::Pool, path: &PathBuf) -> Result<(), Error> {
        let Self(config) = Self::compile(pool).await?;

        tracing::debug!(?config, "writing config to disk");
        config.write(path).map_err(Error::internal_error)
    }
}
