// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(transparent)]
pub struct Url(url::Url);

impl Url {
    pub fn parse(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self(url::Url::parse(url)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Url {
    type Err = url::ParseError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        Self::parse(token)
    }
}

impl<'a> Deserialize<'a> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let url = String::deserialize(deserializer)?;
        Url::parse(&url).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<DB> sqlx::Type<DB> for Url
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
impl<'a, DB> sqlx::Encode<'a, DB> for Url
where
    DB: sqlx::Database,
    String: sqlx::Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.to_string().encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Decode<'a, DB> for Url
where
    DB: sqlx::Database,
    String: sqlx::Decode<'a, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <String as sqlx::Decode<DB>>::decode(value)?;
        Ok(Url::parse(&value)?)
    }
}
