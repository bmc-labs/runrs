// Copyright 2024 bmc::labs GmbH. All rights reserved.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use atmosphere::{table, Read, Schema, Table as _};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema, ToSchema, IntoParams)]
#[table(schema = "public", name = "runners")]
pub struct Runner {
    #[sql(pk)]
    pub id: String,
    pub url: String,
    pub token: String,
    pub description: String,
    pub image: String,
    pub tag_list: String,
    pub run_untagged: bool,
}

impl Runner {
    pub fn update(&mut self, other: Self) -> eyre::Result<()> {
        if self.id != other.id {
            eyre::bail!("Cannot update runner with different ID");
        }

        self.url = other.url;
        self.token = other.token;
        self.description = other.description;
        self.image = other.image;
        self.tag_list = other.tag_list;
        self.run_untagged = other.run_untagged;

        eyre::Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabRunnerConfig {
    #[serde(flatten)]
    pub global_section: GlobalSection,
    pub runners: Vec<Runner>,
}

impl GitLabRunnerConfig {
    pub async fn new(pool: &atmosphere::Pool) -> eyre::Result<Self> {
        let global_section = GlobalSection::default();
        let runners = Runner::find_all(pool).await?;
        Ok(Self {
            global_section,
            runners,
        })
    }

    pub async fn write(&self, path: &PathBuf) -> eyre::Result<()> {
        let config_toml = toml::to_string_pretty(self).map_err(Error::internal_error)?;
        println!("Config toml \n\n{}", config_toml);

        // create directory if it doesn't exist
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        
        let mut file = File::create(path).map_err(Error::internal_error)?;
        file.write_all(config_toml.as_bytes())
            .map_err(Error::internal_error)?;
        Ok(())
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

#[cfg(test)]
impl Runner {
    pub fn for_testing() -> Self {
        Runner {
            id: "42".to_string(),
            url: "https://gitlab.your-company.com".to_string(),
            token: "gltok-warblgarbl".to_string(),
            description: "Knows the meaning of life".to_string(),
            image: "alpine:latest".to_string(),
            tag_list: "test,runner".to_string(),
            run_untagged: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use atmosphere::{query, Create as _, Delete as _, Error, Pool, Read as _, Update as _};
    use pretty_assertions::assert_eq;

    use super::Runner;

    #[sqlx::test]
    async fn create_delete(pool: Pool) -> eyre::Result<()> {
        let mut runner = Runner::for_testing();

        assert!(matches!(
            Runner::find(&runner.id, &pool).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);
        assert_eq!(Runner::find(&runner.id, &pool).await?, runner);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(matches!(
            Runner::find(&runner.id, &pool).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));

        Ok(())
    }

    #[sqlx::test]
    async fn update(pool: Pool) -> eyre::Result<()> {
        let mut runner = Runner::for_testing();

        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        runner.url = "https://gitlab.bmc-labs.com".to_string();
        assert_eq!(runner.save(&pool).await?.rows_affected(), 1);
        assert_eq!(Runner::find(&runner.id, &pool).await?, runner);

        Ok(())
    }

    #[sqlx::test]
    async fn find_all(pool: Pool) -> eyre::Result<()> {
        assert!(Runner::find_all(&pool).await?.is_empty());

        let mut runner = Runner::for_testing();
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        assert_eq!(Runner::find_all(&pool).await?, vec![runner.clone()]);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(Runner::find_all(&pool).await?.is_empty());

        Ok(())
    }
}
