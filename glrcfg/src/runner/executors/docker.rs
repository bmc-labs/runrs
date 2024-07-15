// Copyright 2024 bmc::labs GmbH. All rights reserved.
use serde::{Serialize, Serializer};

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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_images: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_privileged_images: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_pull_policies: Vec<String>,
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
    pub cpu_shares: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub device_cgroup_rules: Vec<String>, // https://docs.docker.com/compose/compose-file/05-services/#device_cgroup_rules
    pub disable_cache: bool,                // written in cli runner creation
    pub disable_entrypoint_overwrite: bool, // written in cli runner creation
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
    pub image: String, // written in cli runner creation
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
    pub network_mtu: u32, // written in cli runner creation, not in gitlab runner docs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    pub oom_kill_disable: bool, // written in cli runner creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adjust: Option<i32>,
    pub privileged: bool, // written in cli runner creation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pull_policy: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_opt: Vec<SecurityOpt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<u32>,
    pub smg_size: u32, // written in cli runner creation, not in gitlub runner docs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Sysctls>, // todo: Implement Sysctls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cert_path: Option<String>,
    pub tls_verify: bool, // written in cli runner creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<String>, // written in cli runner creation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub volumes_from: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    pub wait_for_service_timeout: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub container_labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Services>,
}

impl Default for Docker {
    fn default() -> Self {
        Self {
            allowed_images: Vec::new(),
            allowed_privileged_images: Vec::new(),
            allowed_pull_policies: Vec::new(),
            allowed_services: Vec::new(),
            allowed_privileged_services: Vec::new(),
            cache_dir: None,
            cap_add: Vec::new(),
            cap_drop: Vec::new(),
            cpuset_cpus: None,
            cpuset_mems: None,
            cpu_shares: 1024, // default value as read in the gitlab docs
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
            pull_policy: vec!["always".to_string()], // Default would be "always" as string. Multiple policies are defined as list.
            runtime: None,
            isolation: None,
            security_opt: Vec::new(),
            shm_size: None,
            smg_size: 0,
            sysctls: None,
            tls_cert_path: None,
            tls_verify: false, // default value as read in the gitlab docs
            user: None,
            userns_mode: None,
            volumes: stringvec!["/cache"],
            volumes_from: Vec::new(),
            volume_driver: None,
            wait_for_service_timeout: 30, // default value as read in the gitlab docs
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
pub struct Services {
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

#[derive(Debug)]
pub struct SecurityOpt {
    pub key: String,
    pub value: String,
}

impl Serialize for SecurityOpt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}:{}", self.key, self.value);
        serializer.serialize_str(&s)
    }
}
