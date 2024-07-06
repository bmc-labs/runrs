// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::fmt;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static RUNNER_TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^glrt-\w{20}$").unwrap());

#[derive(Debug, PartialEq, Eq, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
#[error("invalid runner token")]
pub struct RunnerTokenParseError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct RunnerToken(String);

impl RunnerToken {
    pub fn parse<S>(token: S) -> Result<Self, RunnerTokenParseError>
    where
        S: Into<String>,
    {
        let token = token.into();

        if !RUNNER_TOKEN_REGEX.is_match(&token) {
            #[cfg(feature = "tracing")]
            tracing::error!("invalid runner token: {token}");
            return Err(RunnerTokenParseError);
        }

        Ok(Self(token))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RunnerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
