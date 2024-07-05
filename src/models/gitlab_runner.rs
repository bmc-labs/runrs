// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{table, Schema, Table as _};
use glrcfg::runner::{DateTime, Docker, Runner, RunnerToken, Url};
use names::{Generator, Name};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

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
    /// Unique ID of the runner within the GitLab instance
    #[sql(pk)]
    id: u32,
    /// Runner name (default: Docker-style random name)
    #[serde(alias = "description", default = "default_name")]
    #[schema(example = "usain-bolt")]
    name: String,
    /// GitLab instance URL
    #[schema(value_type = String, format = Uri, example = "https://gitlab.your-company.com")]
    url: Url,
    /// Runner token, obtained from the GitLab instance. Format: `glrt-` followed by 20 characters
    /// from the set `[0-9a-f_]`.
    #[schema(value_type = String, example = "glrt-0123456789abcdef____")]
    token: RunnerToken,
    #[schema(value_type = String, format = DateTime, example = "2024-01-01T00:00:00Z")]
    token_obtained_at: DateTime,
    /// Docker image to be used
    #[schema(example = "alpine:latest")]
    docker_image: String,
}

impl GitLabRunner {
    pub fn compatible_with(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<GitLabRunner> for Runner {
    fn from(runner: GitLabRunner) -> Self {
        Self {
            name: runner.name,
            url: runner.url,
            token: runner.token,
            token_obtained_at: runner.token_obtained_at,
            docker: Docker {
                image: runner.docker_image,
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
impl GitLabRunner {
    pub fn for_testing() -> Self {
        GitLabRunner {
            id: 42,
            name: "Knows the meaning of life".to_string(),
            url: Url::parse("https://gitlab.your-company.com").expect("given string is a URL"),
            token: RunnerToken::parse("glrt-0123456789abcdef____")
                .expect("given string is a valid token"),
            token_obtained_at: DateTime::parse("2024-01-01T00:00:00Z")
                .expect("given ISO8601 timestamp is valid"),
            docker_image: "alpine:latest".to_string(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
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

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn create_delete(pool: Pool) -> eyre::Result<()> {
        let mut runner = GitLabRunner::for_testing();

        assert!(matches!(
            GitLabRunner::read(&pool, &runner.id).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::read(&pool, &runner.id).await?, runner);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(matches!(
            GitLabRunner::read(&pool, &runner.id).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn update(pool: Pool) -> eyre::Result<()> {
        let mut runner = GitLabRunner::for_testing();

        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        runner.url = "https://gitlab.bmc-labs.com".parse()?;
        assert_eq!(runner.upsert(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::read(&pool, &runner.id).await?, runner);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn find_all(pool: Pool) -> eyre::Result<()> {
        assert!(GitLabRunner::read_all(&pool).await?.is_empty());

        let mut runner = GitLabRunner::for_testing();
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        assert_eq!(GitLabRunner::read_all(&pool).await?, vec![runner.clone()]);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(GitLabRunner::read_all(&pool).await?.is_empty());

        Ok(())
    }
}
