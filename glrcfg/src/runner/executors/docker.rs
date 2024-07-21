// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::{fmt, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

static SECURITY_OPT_REGEX_STR: &str = r".+:.+";
static SECURITY_OPT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!("^{SECURITY_OPT_REGEX_STR}$"))
        .expect("instantiating SECURITY_OPT_REGEX from given static string must not fail")
});

macro_rules! stringvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// The following settings define the Docker container parameters. Docker-in-Docker as a service,
/// or any container runtime configured inside a job, does not inherit these parameters.
///
/// The, let's call it, _specialty_ here is that the GitLab documentation does not specify a
/// default value for all these parameters, only for some - and for a separate set of them, which
/// is partially overlapping with the set for which defaults are specified in the docs, it produces
/// default values when creating a runner via the `gitlab-runner` CLI. Our default implementation
/// is to produce the same output as the `gitlab-runner` CLI, plus default values where they are
/// specified. The docs for each field tell you which are set and from where we determine the
/// default value; all those without this information in their docs default to the Rust defaults
/// (`None` for [`Option`], etc.) and don't show up by default when serializing a config file.
///
/// Further documentation found in [the GitLab
/// docs](https://docs.gitlab.com/runner/configuration/advanced-configuration.html#the-runnersdocker-section).
#[derive(Debug, Serialize)]
pub struct Docker {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_images: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_privileged_images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pull_policies: Option<Vec<PullPolicy>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_services: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_privileged_services: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cap_add: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cap_drop: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// Default determined from GitLab documentation.
    pub cpu_shares: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<String>,
    /// For more, see: https://docs.docker.com/compose/compose-file/05-services/#device_cgroup_rules
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub device_cgroup_rules: Vec<String>,
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub disable_cache: bool,
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub disable_entrypoint_overwrite: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dns: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dns_search: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra_hosts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub group_add: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helper_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helper_image_flavor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helper_image_autoset_arch_and_os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub image: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// Default determined from `gitlab-runner` CLI runner creation; this field is not documented
    /// in the GitLab docs at all, it _only_ shows up when using the `gitlab-runner` CLI.
    pub network_mtu: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub oom_kill_disable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adjust: Option<i32>,
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub privileged: bool,
    /// Default determined from GitLab documentation.
    #[serde(skip_serializing_if = "MaybeMultiple::is_none")]
    pub pull_policy: MaybeMultiple<PullPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_opt: Vec<SecurityOpt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<u32>,
    /// Default determined from `gitlab-runner` CLI runner creation; this field is not documented
    /// in the GitLab docs at all, it _only_ shows up when using the `gitlab-runner` CLI.
    pub smg_size: u32,
    // TODO(@fabio): Implement Sysctls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Sysctls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cert_path: Option<String>,
    /// Default determined from `gitlab-runner` CLI runner creation and GitLab documentation.
    pub tls_verify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Default determined from `gitlab-runner` CLI runner creation.
    pub volumes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes_from: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    /// Default determined from GitLab documentation.
    pub wait_for_service_timeout: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub container_labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,
}

impl Default for Docker {
    fn default() -> Self {
        Self {
            allowed_images: Vec::new(),
            allowed_privileged_images: Vec::new(),
            allowed_pull_policies: Vec::new().into(),
            allowed_services: Vec::new(),
            allowed_privileged_services: Vec::new(),
            cache_dir: None,
            cap_add: Vec::new(),
            cap_drop: Vec::new(),
            cpuset_cpus: None,
            cpuset_mems: None,
            cpu_shares: 1024,
            cpus: None,
            devices: Vec::new(),
            device_cgroup_rules: Vec::new(),
            disable_cache: false,
            disable_entrypoint_overwrite: false,
            dns: Vec::new(),
            dns_search: Vec::new(),
            extra_hosts: Vec::new(),
            gpus: None,
            group_add: Vec::new(),
            helper_image: None,
            helper_image_flavor: None,
            helper_image_autoset_arch_and_os: None,
            host: None,
            hostname: None,
            image: "alpine:latest".to_string(),
            links: Vec::new(),
            memory: None,
            memory_swap: None,
            memory_reservation: None,
            network_mode: None,
            network_mtu: 0,
            mac_address: None,
            oom_kill_disable: false,
            oom_score_adjust: None,
            privileged: false,
            pull_policy: MaybeMultiple::Some(PullPolicy::Always),
            runtime: None,
            isolation: None,
            security_opt: Vec::new(),
            shm_size: None,
            smg_size: 0,
            sysctls: None,
            tls_cert_path: None,
            tls_verify: false,
            user: None,
            userns_mode: None,
            volumes: stringvec!["/cache"],
            volumes_from: Vec::new(),
            volume_driver: None,
            wait_for_service_timeout: 30,
            container_labels: Vec::new(),
            services: Vec::new(),
        }
    }
}

/// sysctl options for docker
#[derive(Debug, Serialize)]
pub struct Sysctls {}

/// Specify additional services that should be run with the job.
///
/// Visit the [Docker Registry](https://hub.docker.com/) for the list of available images.
/// Each service runs in a separate container and is linked to the job.
/// Further documentation found in the [GitLab Docs](https://archives.docs.gitlab.com/15.11/runner/configuration/advanced-configuration.html#the-runnersdockerservices-section)
#[derive(Debug, Serialize)]
pub struct Service {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
}

/// The image pull policy: `never`, `if-not-present` or `always` (default).
///
/// View details in the [pull policies
/// documentation](https://docs.gitlab.com/runner/executors/docker.html#configure-how-runners-pull-images).
///
/// You can also add [multiple pull
/// policies](https://docs.gitlab.com/runner/executors/docker.html#set-multiple-pull-policies),
/// [retry a failed
/// pull](https://docs.gitlab.com/runner/executors/docker.html#retry-a-failed-pull), or [restrict
/// pull
/// policies](https://docs.gitlab.com/runner/executors/docker.html#allow-docker-pull-policies).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum PullPolicy {
    Always,       // "always"
    IfNotPresent, // "if-not-present"
    Never,        // "never"
}

/// An [`Option`], with in addition to `None` and `Some(T)`, there is `Vec(Vec<T>)`.
///
/// As with a regular [`Option`], the default is `None`. There is also an [`MaybeMultiple::is_none()`]
/// method. Other than that, the API of `MaybeMultiple` is clearly much more limited than that of
/// [`Option`], since we only implement what we need for the library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum MaybeMultiple<T> {
    None,
    Some(T),
    Multiple(Vec<T>),
}

impl<T> MaybeMultiple<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl<T> Default for MaybeMultiple<T> {
    fn default() -> Self {
        Self::None
    }
}

/// Enables the following usage pattern:
///
/// ```rust
/// # use glrcfg::runner::{Docker, PullPolicy, MaybeMultiple};
/// let docker = Docker { pull_policy: PullPolicy::Always.into(), ..Default::default() };
/// # assert_eq!(docker.pull_policy, MaybeMultiple::Some(PullPolicy::Always));
/// ```
impl From<PullPolicy> for MaybeMultiple<PullPolicy> {
    fn from(pull_policy: PullPolicy) -> Self {
        Self::Some(pull_policy)
    }
}

/// Enables the following usage pattern:
///
/// ```rust
/// # use glrcfg::runner::{Docker, PullPolicy, MaybeMultiple};
/// let docker = Docker {
///     pull_policy: vec![PullPolicy::Always, PullPolicy::IfNotPresent].into(),
///     ..Default::default()
/// };
/// # assert_eq!(
/// #     docker.pull_policy,
/// #     MaybeMultiple::Multiple(vec![PullPolicy::Always, PullPolicy::IfNotPresent])
/// # );
/// ```
impl From<Vec<PullPolicy>> for MaybeMultiple<PullPolicy> {
    fn from(pull_policies: Vec<PullPolicy>) -> Self {
        Self::Multiple(pull_policies)
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
#[error("invalid security option; must be a key:value pair")]
pub struct SecurityOptParseError;

/// Security option (`–security-opt` in `docker run`). Must be a `key:value` pair.
///
/// # Example
///
/// ```rust
/// # use glrcfg::runner::SecurityOpt;
/// let security_opt = SecurityOpt::parse("key:value").unwrap();
/// assert_eq!(security_opt.as_str(), "key:value");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SecurityOpt(String);

impl SecurityOpt {
    /// Parses a security option from an `Into<String>`, e.g. a `&str` or `String`.
    pub fn parse<S>(opt: S) -> Result<Self, SecurityOptParseError>
    where
        S: Into<String>,
    {
        let opt = opt.into();

        if !SECURITY_OPT_REGEX.is_match(&opt) {
            #[cfg(feature = "tracing")]
            tracing::error!("invalid security option: {opt}");
            return Err(SecurityOptParseError);
        }

        Ok(Self(opt))
    }

    /// Returns the security option as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SecurityOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SecurityOpt {
    type Err = SecurityOptParseError;

    fn from_str(opt: &str) -> Result<Self, Self::Err> {
        Self::parse(opt)
    }
}

impl<'a> Deserialize<'a> for SecurityOpt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let opt = String::deserialize(deserializer)?;
        Self::parse(opt).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "sqlx")]
impl<DB> sqlx::Type<DB> for SecurityOpt
where
    DB: sqlx::Database,
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as sqlx::Type<DB>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Encode<'a, DB> for SecurityOpt
where
    DB: sqlx::Database,
    String: sqlx::Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'a, DB> sqlx::Decode<'a, DB> for SecurityOpt
where
    DB: sqlx::Database,
    String: sqlx::Decode<'a, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <String as sqlx::Decode<DB>>::decode(value)?;
        Ok(SecurityOpt::parse(value)?)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use test_strategy::proptest;

    use super::{
        MaybeMultiple, PullPolicy, SecurityOpt, SECURITY_OPT_REGEX, SECURITY_OPT_REGEX_STR,
    };

    #[proptest]
    fn parse_valid_security_options(#[strategy(SECURITY_OPT_REGEX_STR)] opt: String) {
        assert_eq!(opt, SecurityOpt::parse(&opt).unwrap().as_str());
    }

    #[proptest]
    fn parse_invalid_security_options(#[filter(|o| !SECURITY_OPT_REGEX.is_match(o))] opt: String) {
        assert!(SecurityOpt::parse(opt).is_err());
    }

    #[test]
    fn pull_policy_serialization() {
        let policy = PullPolicy::Always;
        let serialized = serde_json::to_string(&policy).unwrap();
        assert_eq!(serialized, r#""always""#);

        let policy = MaybeMultiple::from(vec![PullPolicy::Always, PullPolicy::IfNotPresent]);
        let serialized = serde_json::to_string(&policy).unwrap();
        assert_eq!(serialized, r#"["always","if-not-present"]"#);
    }
}
