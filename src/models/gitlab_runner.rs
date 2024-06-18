// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{table, Schema, Table as _};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::glrcfg::{Docker, Runner};

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
    id: String,
    /// Runner name (default: "$(hostname)")
    #[serde(alias = "description")]
    name: String,
    /// GitLab instance URL
    url: String,
    /// Runner token
    token: String,
    /// Docker image to be used
    docker_image: String,
}

impl GitLabRunner {
    pub fn update(&mut self, other: Self) -> eyre::Result<()> {
        if self.id != other.id {
            eyre::bail!("Cannot update runner with different ID");
        }

        self.name = other.name;
        self.url = other.url;
        self.token = other.token;
        self.docker_image = other.docker_image;

        Ok(())
    }
}

impl From<GitLabRunner> for Runner {
    fn from(runner: GitLabRunner) -> Self {
        Runner {
            name: runner.name,
            url: runner.url,
            token: runner.token,
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
            id: "42".to_string(),
            name: "Knows the meaning of life".to_string(),
            url: "https://gitlab.your-company.com".to_string(),
            token: "gltok-warblgarbl".to_string(),
            docker_image: "alpine:latest".to_string(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
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
            GitLabRunner::find(&runner.id, &pool).await,
            Err(Error::Query(query::QueryError::NotFound(
                sqlx::Error::RowNotFound,
            )))
        ));
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::find(&runner.id, &pool).await?, runner);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(matches!(
            GitLabRunner::find(&runner.id, &pool).await,
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

        runner.url = "https://gitlab.bmc-labs.com".to_string();
        assert_eq!(runner.save(&pool).await?.rows_affected(), 1);
        assert_eq!(GitLabRunner::find(&runner.id, &pool).await?, runner);

        Ok(())
    }

    #[sqlx::test(migrator = "crate::MIGRATOR")]
    async fn find_all(pool: Pool) -> eyre::Result<()> {
        assert!(GitLabRunner::find_all(&pool).await?.is_empty());

        let mut runner = GitLabRunner::for_testing();
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);

        assert_eq!(GitLabRunner::find_all(&pool).await?, vec![runner.clone()]);
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert!(GitLabRunner::find_all(&pool).await?.is_empty());

        Ok(())
    }
}
