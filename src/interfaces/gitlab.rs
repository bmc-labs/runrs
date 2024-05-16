// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::Read;
use eyre::Context;
use serde::{Deserialize, Serialize};

use crate::model::Runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub global: GlobalConfig,
    pub runners: Vec<RunnerConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub concurrent: i32,
    pub check_interval: i32,
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

pub async fn print_cfg_toml(pool: atmosphere::Pool) -> eyre::Result<()> {
    let runners = get_runners(pool).await?;
    let global_config = GlobalConfig {
        concurrent: 4,
        check_interval: 3,
    };
    let runner_config: Vec<RunnerConfig> = create_runner_config(runners).await?;
    write_config_toml(global_config, runner_config);
    Ok(())
}

async fn get_runners(pool: atmosphere::Pool) -> eyre::Result<Vec<Runner>> {
    Runner::find_all(&pool)
        .await
        .wrap_err("Failed to retrieve runners from database")
}

async fn create_runner_config(runners: Vec<Runner>) -> eyre::Result<Vec<RunnerConfig>> {
    let configs = runners
        .into_iter()
        .map(|runner| {
            RunnerConfig {
                id: runner.id,
                name: "runner".to_owned(),
                url: runner.url,
                token: runner.token,
                executor: "docker".to_owned(),
                description: Some(runner.description),
                tag_list: runner.tag_list.split(',').map(String::from).collect(),
                run_untagged: runner.run_untagged,
                shell: Some("bash".to_owned()), // Default shell
                docker: DockerConfig {
                    image: runner.image.to_owned(),
                },
            }
        })
        .collect();
    Ok(configs)
}

fn write_config_toml(glcfg: GlobalConfig, runners_cfg: Vec<RunnerConfig>) {
    let config = Config {
        global: glcfg,
        runners: runners_cfg,
    };
    let config_toml: String = toml::to_string_pretty(&config).expect("Failed serializing config");
    println!("Config toml \n\n{}", config_toml);
}
