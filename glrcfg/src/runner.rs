// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;

/// The following settings define the Docker container parameters. Docker-in-Docker as a service,
/// or any container runtime configured inside a job, does not inherit these parameters.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, Serialize)]
pub struct Docker {
    pub image: String,
}

impl Default for Docker {
    fn default() -> Self {
        Self {
            image: "alpine:latest".to_string(),
        }
    }
}

/// Defines one runner.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-global-section).
#[derive(Debug, Serialize)]
pub struct Runner {
    pub name: String,
    pub url: String,
    pub token: String,
    pub executor: String,
    pub docker: Docker,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            url: "https://gitlab.com/".to_string(),
            token: "".to_string(),
            executor: "docker".to_string(),
            docker: Default::default(),
        }
    }
}
