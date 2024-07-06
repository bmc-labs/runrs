// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;

/// The following settings define the Docker container parameters. Docker-in-Docker as a service,
/// or any container runtime configured inside a job, does not inherit these parameters.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, Serialize)]
pub struct Docker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_images: Option<String>,
    pub image: String,
}

impl Default for Docker {
    fn default() -> Self {
        Self {
            allowed_images: None,
            image: "alpine:latest".to_string(),
        }
    }
}
