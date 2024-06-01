// Copyright 2024 bmc::labs GmbH. All rights reserved.

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

const DEFAULT_VALIDITY_PERIOD_DAYS: i64 = 90;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // default fields
    iat: usize,  // issued at - UTC timestamp in seconds
    exp: usize,  // expiration time - UTC timestamp in seconds
    iss: String, // issuer
    // custom fields
    org: String, // organization
}

impl Claims {
    pub fn new(validity_period_days: Option<i64>) -> eyre::Result<Self> {
        let now = Utc::now();
        let exp = now
            .checked_add_signed(TimeDelta::days(
                validity_period_days.unwrap_or(DEFAULT_VALIDITY_PERIOD_DAYS),
            ))
            .ok_or(eyre::eyre!("could not calculate expiration time"))?;

        Ok(Self {
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
            iss: "peripheral".to_string(),
            org: "bmc::labs".to_string(),
        })
    }
}

pub fn init_secret() -> eyre::Result<String> {
    let Ok(secret) = std::env::var("SECRET") else {
        tracing::error!("SECRET not set - aborting");
        eyre::bail!("SECRET not set");
    };

    let token = encode(
        &Header::new(Algorithm::HS512),
        &Claims::new(None)?,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    tracing::info!(?token, "Generated token");

    Ok(secret)
}
