// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod docker;

pub use docker::{Docker, Services, Sysctls};
use serde::Serialize;

/// The following executors are available.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-executors).
///
/// ### Note
///
/// Perhaps you noticed we don't support all executors from the list in the GitLab docs. That is
/// intentional. The executor `docker-windows` is on the roadmap, both `parallels` and `virtualbox`
/// are still up for debate. We don't plan to ever support `docker+machine`, since the underlying
/// technology - "Docker Machine" - is deprecated.
#[derive(Debug, Serialize)]
#[serde(tag = "executor", rename_all = "lowercase")]
pub enum Executor {
    Shell,
    Docker { docker: Docker },
    Ssh,
    Kubernetes,
}
