// Copyright 2024 bmc::labs GmbH. All rights reserved.

mod docker;

pub use docker::{Docker, MaybeMultiple, PullPolicy, SecurityOpt, Service, Sysctls};
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
//
// This `#[allow]` turning off the clippy warning for large size differences between enum variants
// is needed because `Docker` is huge, but using `Box<Docker>` would mean that users would have to
// call `Box::new(docker)` with their config, which isn't exactly ergonomic.
//
// TODO(@florian): When other variants are added, check if the clippy warning can be turned back on.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize)]
#[serde(tag = "executor", rename_all = "lowercase")]
pub enum Executor {
    Shell,
    Docker { docker: Docker },
}
