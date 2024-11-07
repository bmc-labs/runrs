// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fmt, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static RUNNER_TOKEN_REGEX_STR: &str = r"glrt-[\w-]{16,32}"; // note the hyphen
static RUNNER_TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!("^{RUNNER_TOKEN_REGEX_STR}$"))
        .expect("instantiating RUNNER_TOKEN_REGEX from given static string must not fail")
});

#[derive(Debug, PartialEq, Eq, Error)]
#[error("invalid runner token `{0}`; must look like glrt-0123456789_abcdefXYZ")]
pub struct RunnerTokenParseError(String);

/// GitLab uses various kinds of tokens for authentication. When registering a runner via the
/// GitLab UI, a runner token is generated and presented to the user. It must then be provided to
/// the `gitlab-runner`  binary via the `--token` argument, or, as is the intention here, via the
/// configuration file.
///
/// Valid tokens start with `glrt-`, followed by at least 16 and at most 32 alphanumeric
/// characters, plus underscore. An alphanumeric character is one which matches the regular
/// expression `[a-zA-Z0-9_]` (note the underscore being part of the allowed characters).
///
/// # Example
///
/// ```rust
/// # use glrcfg::runner::RunnerToken;
/// let runner_token = RunnerToken::parse("glrt-0123456789_abcdefXYZ").unwrap();
/// assert_eq!(runner_token.as_str(), "glrt-0123456789_abcdefXYZ");
/// assert!(RunnerToken::parse("warblgarbl").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct RunnerToken(String);

impl RunnerToken {
    /// Parses a runner token from an `Into<String>`, e.g. a `&str` or `String`.
    pub fn parse<S>(token: S) -> Result<Self, RunnerTokenParseError>
    where
        S: Into<String>,
    {
        let token = token.into();

        if !RUNNER_TOKEN_REGEX.is_match(&token) {
            #[cfg(feature = "tracing")]
            tracing::error!("invalid runner token: {token}");
            return Err(RunnerTokenParseError(token));
        }

        Ok(Self(token))
    }

    /// Returns the runner token as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RunnerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for RunnerToken {
    type Err = RunnerTokenParseError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        Self::parse(token)
    }
}

impl<'a> Deserialize<'a> for RunnerToken {
    fn deserialize<D>(deserializer: D) -> Result<RunnerToken, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let token = String::deserialize(deserializer)?;
        RunnerToken::parse(token).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<DB> sqlx::Type<DB> for RunnerToken
where
    DB: sqlx::Database,
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as sqlx::Type<DB>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Encode<'a, DB> for RunnerToken
where
    DB: sqlx::Database,
    String: sqlx::Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Decode<'a, DB> for RunnerToken
where
    DB: sqlx::Database,
    String: sqlx::Decode<'a, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <String as sqlx::Decode<DB>>::decode(value)?;
        Ok(RunnerToken::parse(value)?)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use test_strategy::proptest;

    use super::{RunnerToken, RUNNER_TOKEN_REGEX, RUNNER_TOKEN_REGEX_STR};

    #[proptest]
    fn parse_valid_runner_tokens(#[strategy(RUNNER_TOKEN_REGEX_STR)] token: String) {
        assert_eq!(token, RunnerToken::parse(&token).unwrap().as_str());
    }

    #[proptest]
    fn parse_invalid_runner_tokens(#[filter(|t| !RUNNER_TOKEN_REGEX.is_match(t))] token: String) {
        assert!(RunnerToken::parse(token).is_err());
    }

    #[test]
    fn parse_known_valid_runner_tokens() {
        let token = "glrt-ZJAbdjMq-ViUVE_zd1VD";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-6tS-PTLZLdzKDq-5mZvx";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-dmayYi77SP-5fTibBcH3";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-4dFJv-Y6cBpEdWV774zx";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-YypxpG7h-rhrViUVar6F";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-t1_NQbUXcKbPbCkJzzVDnmu";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-Cm8DYQiTeGJMVzcW4hyx";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());

        let token = "glrt-t1_CkM3EZEjJ84ts_tYyVCB";
        assert_eq!(token, RunnerToken::parse(token).unwrap().as_str());
    }
}
