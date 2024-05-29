// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::path::PathBuf;

use atmosphere::Read;
use eyre::WrapErr;
use serde::Serialize;

use super::GitLabRunner;

#[derive(Debug, Serialize)]
pub struct GlobalSection {
    pub concurrent: u32,
    pub check_interval: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self {
            concurrent: 4,
            check_interval: 3,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GitLabRunnerConfig {
    #[serde(flatten)]
    pub global_section: GlobalSection,
    pub runners: Vec<GitLabRunner>,
}

impl GitLabRunnerConfig {
    pub async fn compile(pool: &atmosphere::Pool) -> eyre::Result<Self> {
        let global_section = GlobalSection::default();
        let runners = GitLabRunner::find_all(pool).await?;

        Ok(Self {
            global_section,
            runners,
        })
    }

    pub async fn write(pool: &atmosphere::Pool, path: &PathBuf) -> eyre::Result<()> {
        let config = Self::compile(pool).await?;
        let config_toml =
            toml::to_string_pretty(&config).wrap_err("could not serialize to TOML")?;

        tracing::debug!(?config_toml, "writing config to disk");
        std::fs::write(path, config_toml).wrap_err(format!("unable to write to file at {path:?}"))
    }
}
