// Copyright 2024 bmc::labs GmbH. All rights reserved.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use url::Url;
use validator::Validate;

static GO_DURATION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[+-]?(\d+(h|m|s|ms|us|µs|ns))+$").unwrap());

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
#[derive(Debug, Validate, Serialize)]
pub struct GlobalSection {
    #[validate(range(min = 1))]
    pub concurrent: u32,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub check_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Url>,
    #[validate(regex(path = *GO_DURATION_REGEX))]
    pub connection_max_age: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<Url>,
    pub shutdown_timeout: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self {
            concurrent: 1,
            log_level: LogLevel::Error,
            log_format: LogFormat::Json,
            check_interval: 3,
            sentry_dsn: None,
            connection_max_age: "15m".to_string(),
            listen_address: None,
            shutdown_timeout: 30,
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_default() {
        let global_section = GlobalSection::default();

        assert!(global_section.validate().is_ok());

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
