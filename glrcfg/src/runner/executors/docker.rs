// Copyright 2024 bmc::labs GmbH. All rights reserved.

use serde::Serialize;

macro_rules! stringvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// The following settings define the Docker container parameters. Docker-in-Docker as a service,
/// or any container runtime configured inside a job, does not inherit these parameters.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-runnersdocker-section).
#[derive(Debug, Serialize)]
pub struct Docker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_images: Option<String>,
    pub disable_cache: bool,
    pub disable_entrypoint_overwrite: bool,
    pub image: String,
    pub network_mtu: u32,
    pub oom_kill_disable: bool,
    pub privileged: bool,
    pub smg_size: u32,
    pub tls_verify: bool,
    pub volumes: Vec<String>,
}

impl Default for Docker {
    fn default() -> Self {
        Self {
            allowed_images: None,
            disable_cache: false,
            disable_entrypoint_overwrite: false,
            image: "alpine:latest".to_string(),
            network_mtu: 0,
            oom_kill_disable: false,
            privileged: false,
            smg_size: 0,
            tls_verify: false,
            volumes: stringvec!["/cache"],
        }
    }
}
