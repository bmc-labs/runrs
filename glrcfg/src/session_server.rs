// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;
use url::Url;

/// The `[session_server]` section lets users interact with jobs, for example, in the interactive
/// web terminal.
///
/// The `[session_server]` section should be specified at the root level, not per runner. It should
/// be defined outside the `[[runners]]` section.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-session_server-section).
#[derive(Debug, Serialize)]
pub struct SessionServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_address: Option<Url>,
    pub session_timeout: u32,
}

impl Default for SessionServer {
    fn default() -> Self {
        Self {
            listen_address: None,
            advertise_address: None,
            session_timeout: 1800,
        }
    }
}
