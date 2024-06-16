// Copyright 2024 bmc::labs GmbH. All rights reserved.

use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

/// Initializes backtracing and error handling capabilities.
pub fn init() -> eyre::Result<()> {
    // Logs in prod environments are often expensive,
    // incurring per-MB costs in some cases (e.g. AWS).
    // We therefore default to ERROR level for eveything
    // except runrs itself, which defaults to WARN.
    let filter =
        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::try_new("error,runrs=warn")?);

    let subscriber = tracing_subscriber::fmt().with_env_filter(filter);

    match std::env::var("LOG_FMT") {
        Ok(fmt) if fmt == "json" => subscriber.json().finish().init(),
        _ => subscriber.finish().init(),
    }

    Ok(())
}
