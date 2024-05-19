// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes backtracing and error handling capabilities.
pub fn init() -> eyre::Result<()> {
    const BT_ENVVAR: &str = "RUST_LIB_BACKTRACE";
    if std::env::var(BT_ENVVAR).is_err() {
        std::env::set_var(BT_ENVVAR, "1")
    }

    // set up format layer with filtering for tracing
    const LG_ENVVAR: &str = "RUST_LOG";
    if std::env::var(LG_ENVVAR).is_err() {
        std::env::set_var(LG_ENVVAR, "error,runrs=debug")
    }

    let filter = Targets::from_str(
        std::env::var("RUST_LOG")
            .as_deref()
            .unwrap_or("error,runrs=debug"),
    )?;

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        // TODO(flrn): turn on JSON once we start logging to a service
        // .json()
        .finish()
        .with(filter)
        .init();

    Ok(())
}
