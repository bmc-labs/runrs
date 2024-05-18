// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::Read;
use eyre::Context;
use serde::{Deserialize, Serialize};

use crate::model::Runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabRunnerConfig {
    #[serde(flatten)]
    pub global_section: GlobalSection,
    pub runners: Vec<Runners>,
}

impl Config {
    pub async fn new() -> eyre::Result<Self> {
        let global_section = GlobalSection::default();
        let runners = Runner::find_all(...).await.wrap_err("warbl")?;

        Self { global_section, runners }
    }
    
    pub async fn write(&self) -> eyre::Result<()> {
        let config_toml = toml::to_string_pretty(self).wrap_err("Failed serializing config")?;
        
        println!("Config toml \n\n{}", config_toml);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalSection {
    pub concurrent: u32,
    pub check_interval: u32,
}

impl Default for GlobalSection {
    fn default() -> Self {
        Self {
            concurrent: 4,
            check_interval: 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub token: String,
    pub executor: String,
    pub description: Option<String>,
    pub tag_list: Vec<String>,
    pub run_untagged: bool,
    pub shell: Option<String>,
    pub docker: DockerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerConfig {
    pub image: String,
}

impl From<Runner> for RunnerConfig {
    fn from(r: Runner) -> Self {
        RunnerConfig {
            id: r.id,
            name: "runner".to_owned(),
            url: r.url,
            token: r.token,
            executor: "docker".to_owned(),
            description: Some(r.description),
            tag_list: r.tag_list.split(',').map(String::from).collect(),
            run_untagged: r.run_untagged,
            shell: Some("bash".to_owned()), // Default shell
            docker: DockerConfig {
                image: r.image.to_owned(),
            },
        }
    }
}
