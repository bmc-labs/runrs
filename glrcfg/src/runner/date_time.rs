// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// A datetime type that serializes to and from ISO8601 strings. Based on
/// [`chrono::DateTime<chrono::Utc>`]. Used as timestamp for the `token_obtained_at` and
/// `token_expires_at` fields in [`Runner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn parse<S>(iso8601: S) -> Result<Self, chrono::ParseError>
    where
        S: Into<String>,
    {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(&iso8601.into())?.with_timezone(&chrono::Utc),
        ))
    }

    pub fn to_iso8601(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}

impl FromStr for DateTime {
    type Err = chrono::ParseError;

    fn from_str(iso8601: &str) -> Result<Self, Self::Err> {
        Self::parse(iso8601)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_iso8601().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let date_time = String::deserialize(deserializer)?;
        DateTime::parse(date_time).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<DB> sqlx::Type<DB> for DateTime
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
impl<'a, DB> sqlx::Encode<'a, DB> for DateTime
where
    DB: sqlx::Database,
    String: sqlx::Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.to_iso8601().encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Decode<'a, DB> for DateTime
where
    DB: sqlx::Database,
    String: sqlx::Decode<'a, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <String as sqlx::Decode<DB>>::decode(value)?;
        Ok(DateTime::parse(value)?)
    }
}
