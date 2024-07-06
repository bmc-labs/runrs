// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{num::NonZeroU32, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;
use url::Url;

static GOLANG_DURATION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([+-]?(\d+(h|m|s|ms|us|µs|ns))+|0)$").unwrap());

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

#[derive(Debug, PartialEq, Eq, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
#[error("invalid Golang duration (which look like 15m, 1h, 1h15m, etc.)")]
pub struct GolangDurationParseError;

/// The Golang standard library [has a `Duration` type](https://pkg.go.dev/time#Duration), which
/// has a function called `ParseDuration` that accepts formatted strings like these: `15m` for 15
/// minutes, `1h` for 1 hour, `1h15m` for 1 hour and 15 minutes. This type enforces that format.
#[derive(Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct GolangDuration(String);

impl GolangDuration {
    pub fn parse<S>(duration: S) -> Result<Self, GolangDurationParseError>
    where
        S: Into<String>,
    {
        let duration = duration.into();

        if !GOLANG_DURATION_REGEX.is_match(&duration) {
            #[cfg(feature = "tracing")]
            tracing::error!("invalid Golang duration: {duration}");
            return Err(GolangDurationParseError);
        }

        Ok(Self(duration))
    }
}

impl FromStr for GolangDuration {
    type Err = GolangDurationParseError;

    fn from_str(duration: &str) -> Result<Self, Self::Err> {
        Self::parse(duration)
    }
}

/// These settings are global. They apply to all runners.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, Serialize)]
pub struct GlobalSection {
    pub concurrent: NonZeroU32,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub check_interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry_dsn: Option<Url>,
    pub connection_max_age: GolangDuration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<Url>,
    pub shutdown_timeout: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self {
            concurrent: NonZeroU32::new(1).expect("1 is not zero"),
            log_level: LogLevel::Error,
            log_format: LogFormat::Json,
            check_interval: 3,
            sentry_dsn: None,
            connection_max_age: GolangDuration::parse("15m").expect("15m is a valid duration"),
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