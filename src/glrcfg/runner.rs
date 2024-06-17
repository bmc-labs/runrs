// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;

/// Defines one runner. Documentation of fields found here:
/// https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-runners-section
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
            docker: Docker {
                image: "alpine:latest".to_string(),
            },
        }
    }
}

/// The following settings define the Docker container parameters. Docker-in-Docker as a service,
/// or any container runtime configured inside a job, does not inherit these parameters.
/// Documentation of fields found here:
/// https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-runnersdocker-section
#[derive(Debug, Serialize)]
pub struct Docker {
    pub image: String,
}
