// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::path::PathBuf;

use atmosphere::Read;
use eyre::WrapErr;

use super::GitLabRunner;
use crate::glrcfg::{Config, Runner};

#[derive(Debug)]
pub struct GitLabRunnerConfig(Config);

impl GitLabRunnerConfig {
    pub async fn compile(pool: &atmosphere::Pool) -> eyre::Result<Self> {
        let runners = GitLabRunner::find_all(pool)
            .await?
            .into_iter()
            .map(Runner::from)
            .collect();

        let config = Config::builder().with_runners(runners).finish();

        Ok(Self(config))
    }

    pub async fn write(pool: &atmosphere::Pool, path: &PathBuf) -> eyre::Result<()> {
        let Self(config) = Self::compile(pool).await?;

        tracing::debug!(?config, "writing config to disk");
        config
            .write(path)
            .wrap_err(format!("unable to write to file at {path:?}"))
    }
}
