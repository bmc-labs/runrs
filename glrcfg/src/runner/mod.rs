// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod date_time;
mod docker;
mod runner_token;
mod url;

pub use date_time::DateTime;
pub use docker::Docker;
pub use runner_token::{RunnerToken, RunnerTokenParseError};
use serde::Serialize;
pub use url::Url;

/// Defines one runner.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-runners-section).
#[derive(Debug, Serialize)]
pub struct Runner {
    pub id: u32,
    pub name: String,
    pub url: Url,
    pub token: RunnerToken,
    /// Timestamp of when the token was "obtained". This field is undocumented in [the GitLab docs
    /// for the GitLab Runners configuration
    /// file](https://docs.gitlab.com/runner/configuration/advanced-configuration.html) and it is
    /// nowhere to be found in [the GitLab API docs](https://docs.gitlab.com/ee/api/runners.html),
    /// but it is present in the configuration files generated by the `gitlab-runner` binary. It's
    /// the timestamp of when the token was first handed to the service processing it (and using
    /// this library).
    pub token_obtained_at: DateTime,
    /// Timestamp of when the token will "expire". While being undocumented in the GitLab Runner
    /// docs for the configuration file, it can be obtained through [the GitLab
    /// API](https://docs.gitlab.com/ee/api/runners.html#verify-authentication-for-a-registered-runner)
    /// and can be set here.
    pub token_expires_at: DateTime,
    pub limit: u32,
    pub executor: String,
    pub builds_dir: String,
    pub cache_dir: String,
    /// Used to set environment variables for a runner or job. Example: `["FOO=bar", "BAZ=qux"]`
    pub environment: Vec<String>,
    pub request_concurrency: u32,
    pub output_limit: u32,
    pub docker: Docker,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            id: 42,
            name: "default".to_string(),
            url: Url::parse("https://gitlab.com/").expect("given string is a URL"),
            token: RunnerToken::parse("glrt-0123456789abcdef____")
                .expect("given string is a valid token"),
            token_obtained_at: DateTime::now(),
            token_expires_at: DateTime::parse("0001-01-01T00:00:00Z")
                .expect("given string is a valid ISO8601 timestamp"),
            limit: 0,
            executor: "docker".to_string(),
            builds_dir: "".to_string(),
            cache_dir: "".to_string(),
            environment: vec![],
            request_concurrency: 1,
            output_limit: 4096,
            docker: Default::default(),
        }
    }
}

//  1   │ concurrent = 1
//  2   │ check_interval = 0
//  3   │ shutdown_timeout = 0
//  4   │
//  5   │ [session_server]
//  6   │   session_timeout = 1800
//  7   │
//  8   │ [[runners]]
//  9   │   name = "wntrmt"
// 10   │   url = "https://gitlab.bmc-labs.com"
// 11   │   id = 18
// 12   │   token = "glrt-V4yPR_ePd2WsSoPqx9t9"
// 13   │   token_obtained_at = 2024-06-22T02:25:56Z
// 14   │   token_expires_at = 0001-01-01T00:00:00Z
// 15   │   executor = "docker"
// 16   │   [runners.custom_build_dir]
// 17   │   [runners.cache]
// 18   │     MaxUploadedArchiveSize = 0
// 19   │     [runners.cache.s3]
// 20   │     [runners.cache.gcs]
// 21   │     [runners.cache.azure]
// 22   │   [runners.docker]
// 23   │     tls_verify = false
// 24   │     image = "alpine:latest"
// 25   │     privileged = false
// 26   │     disable_entrypoint_overwrite = false
// 27   │     oom_kill_disable = false
// 28   │     disable_cache = false
// 29   │     volumes = ["/cache"]
// 30   │     shm_size = 0
// 31   │     network_mtu = 0
