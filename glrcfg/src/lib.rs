// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod global_section;
mod runner;

use std::path;

pub use global_section::{
    GlobalSection, GolangDuration, GolangDurationParseError, LogFormat, LogLevel,
};
pub use runner::{Docker, Runner};
use serde::Serialize;

/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub global_section: GlobalSection,
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
    global_section: GlobalSection,
    runners: Vec<Runner>,
}

impl ConfigBuilder {
    pub fn with_runners(mut self, runners: Vec<Runner>) -> Self {
        self.runners = runners;
        self
    }

    pub fn finish(self) -> Config {
        Config {
            global_section: self.global_section,
            runners: self.runners,
        }
    }
}
