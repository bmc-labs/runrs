// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod global;
pub mod runner;
pub mod session_server;

use std::path;

pub use global::{GlobalSection, GolangDuration, GolangDurationParseError, LogFormat, LogLevel};
use runner::Runner;
use serde::Serialize;
use session_server::SessionServer;

/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html).
#[derive(Debug, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub global: GlobalSection,
    pub session_server: SessionServer,
    pub runners: Vec<Runner>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn write<P>(&self, path: P) -> std::io::Result<()>
    where
        P: Into<path::PathBuf> + AsRef<path::Path>,
    {
        let config_toml = toml::to_string_pretty(&self).expect("could not serialize to TOML");

        #[cfg(feature = "tracing")]
        tracing::debug!(?config_toml, "writing config to disk");
        std::fs::write(path, config_toml)
    }
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    global: GlobalSection,
    session_server: SessionServer,
    runners: Vec<Runner>,
}

impl ConfigBuilder {
    pub fn with_runners(mut self, runners: Vec<Runner>) -> Self {
        self.runners = runners;
        self
    }

    pub fn build(self) -> Config {
        Config {
            global: self.global,
            session_server: self.session_server,
            runners: self.runners,
        }
    }
}
