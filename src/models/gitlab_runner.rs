// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{table, Schema, Table as _};
use glrcfg::runner::{DateTime, Docker, Executor, Runner, RunnerToken, Url};
use names::{Generator, Name};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

macro_rules! stringvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn default_name() -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    generator.next().unwrap_or_else(|| "usain-bolt".to_string())
}

/// Public API for configuring a single CI/CD job executor, not the GitLab Runner service.
///
/// GitLab publish a service binary they refer to as "GitLab Runner". You can install it locally or
/// on you server [as per its documentation](https://docs.gitlab.com/runner/install/). This binary
/// is, however, *not* the CI/CD job executor; rather, it _manages_ the executors. As such, when you
/// "register a runner" (as per [their documentation](https://docs.gitlab.com/runner/register/)),
/// you use the `gitlab-runner` binary to do so.
///
/// The `GitLabRunner` struct replicates the API of the `gitlab-runner` binary, albeit exposing a
/// smaller configuration surface. In other words: if you run `gitlab-runner register --help`, you
/// get a list of options. We support a subset of those options, and those which are supported are
/// named the same here as they are in `gitlab-runner`, except in `snake_case` instead of
/// `kebab-case`. For example, `--docker-image` becomes `docker_image`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema, ToSchema, IntoParams)]
#[table(schema = "public", name = "gitlab_runners")]
pub struct GitLabRunner {
    #[sql(pk)]
    #[serde(default = "Uuid::new_v4")]
    #[schema(value_type = String, format = Uuid, example = "be924fdd-fb28-468c-8c70-1f0ed3af4485")]
    uuid: Uuid,
    /// ID of the runner within the GitLab instance; unique for that GitLab instance
    id: u32,
    /// Runner name (default: Docker-style random name)
    #[serde(alias = "description", default = "default_name")]
    #[schema(example = "usain-bolt")]
    name: String,
    /// GitLab instance URL
    #[schema(value_type = String, format = Uri, example = "https://gitlab.your-company.com")]
    url: Url,
    /// Runner token, obtained from the GitLab instance. See [documentation of the `glrcfg`
    /// crate](https://docs.rs/glrcfg/latest/glrcfg/runner/struct.RunnerToken.html) for details.
    #[schema(value_type = String, example = "glrt-0123456789_abcdefXYZ")]
    token: RunnerToken,
    #[serde(default = "DateTime::now")]
    #[schema(value_type = String, format = DateTime, example = "2023-08-23T23:23:23Z")]
    token_obtained_at: DateTime,
    /// Docker image to be used
    #[schema(example = "alpine:latest")]
    docker_image: String,
}

impl GitLabRunner {
    pub fn compatible_with(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl From<GitLabRunner> for Runner {
    fn from(runner: GitLabRunner) -> Self {
        Self {
            name: runner.name,
            url: runner.url,
            token: runner.token,
            token_obtained_at: runner.token_obtained_at,
            executor: Executor::Docker {
                docker: Docker {
                    image: runner.docker_image,
                    // connect the docker socket from the host into all runner containers, enabling
                    // them to access the host's docker daemon for pulling and pushing images
                    volumes: stringvec!["/var/run/docker.sock:/var/run/docker.sock", "/cache"],
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
impl GitLabRunner {
    pub fn for_testing() -> Self {
        GitLabRunner {
            uuid: Uuid::new_v4(),
            id: 42,
            name: "Knows the meaning of life".to_string(),
            url: Url::parse("https://gitlab.your-company.com").expect("given string is a URL"),
            token: RunnerToken::parse("glrt-0123456789_abcdefXYZ")
                .expect("given string is a valid token"),
            token_obtained_at: DateTime::parse("2023-08-23T23:23:23Z")
                .expect("given ISO8601 timestamp is valid"),
            docker_image: "alpine:latest".to_string(),
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = Url::parse(url).expect("given string is not a URL");
    }
}

#[cfg(test)]
mod tests {
    use atmosphere::{query, Create as _, Delete as _, Error, Pool, Read as _, Update as _};
    use pretty_assertions::assert_eq;

    use super::GitLabRunner;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn create_delete(pool: Pool) -> Result<()> {
        let mut runner = GitLabRunner::for_testing();

        assert!(matches!(
            GitLabRunner::read(&pool, &runner.uuid).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::read(&pool, &runner.uuid).await?, runner);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(matches!(
            GitLabRunner::read(&pool, &runner.uuid).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn update(pool: Pool) -> Result<()> {
        let mut runner = GitLabRunner::for_testing();

        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        runner.url = "https://gitlab.bmc-labs.com".parse()?;
        assert_eq!(runner.upsert(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::read(&pool, &runner.uuid).await?, runner);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn find_all(pool: Pool) -> Result<()> {
        assert!(GitLabRunner::read_all(&pool).await?.is_empty());

        let mut runner = GitLabRunner::for_testing();
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        assert_eq!(GitLabRunner::read_all(&pool).await?, vec![runner.clone()]);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(GitLabRunner::read_all(&pool).await?.is_empty());

        Ok(())
    }
}
