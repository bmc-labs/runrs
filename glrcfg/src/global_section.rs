// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;
use typed_builder::TypedBuilder;
use url::Url;

/// Defines the log level. Options are `debug`, `info`, `warn`, `error`, `fatal`, and `panic`. This
/// setting has lower priority than the level set by the command-line arguments `--debug`, `-l`, or
/// `--log-level`.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

/// Specifies the log format. Options are `runner`, `text`, and `json`. This setting has lower
/// priority than the format set by command-line argument `--log-format`. The default value is
/// `runner`, which contains ANSI escape codes for coloring.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Runner,
    Text,
    Json,
}

/// These settings are global. They apply to all runners.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, TypedBuilder, Serialize)]
pub struct GlobalSection {
    #[builder(default = 1)]
    pub concurrent: u32,
    #[builder(default = LogLevel::Error)]
    pub log_level: LogLevel,
    #[builder(default = LogFormat::Json)]
    pub log_format: LogFormat,
    #[builder(default = 3)]
    pub check_interval: u32,
    #[builder(default)]
    pub sentry_dsn: Option<Url>,
    #[builder(default = "15m".to_string())]
    pub connection_max_age: String,
    #[builder(default)]
    pub listen_address: Option<Url>,
    #[builder(default = 30)]
    pub shutdown_timeout: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_default() {
        let global_section = GlobalSection::default();

        assert_eq!(global_section.concurrent, 1);
        assert_eq!(global_section.log_level, LogLevel::Error);
        assert_eq!(global_section.log_format, LogFormat::Json);
        assert_eq!(global_section.check_interval, 3);
        assert_eq!(global_section.sentry_dsn, None);
        assert_eq!(global_section.connection_max_age, "15m");
        assert_eq!(global_section.listen_address, None);
        assert_eq!(global_section.shutdown_timeout, 30);

        let toml = toml::to_string_pretty(&global_section).expect("could not serialize to TOML");

        assert_eq!(
            toml,
            indoc::indoc! {r#"
                concurrent = 1
                log_level = "error"
                log_format = "json"
                check_interval = 3
                connection_max_age = "15m"
                shutdown_timeout = 30
            "#}
        );
    }
}
