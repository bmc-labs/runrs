// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::{table, Schema, Table as _};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema, ToSchema, IntoParams)]
#[table(schema = "public", name = "gitlab_runners")]
pub struct GitLabRunner {
    #[sql(pk)]
    pub id: String,
    pub url: String,
    pub token: String,
    pub description: String,
    pub image: String,
    pub tag_list: String,
    pub run_untagged: bool,
}

impl GitLabRunner {
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

        Ok(())
    }
}

#[cfg(test)]
impl GitLabRunner {
    pub fn for_testing() -> Self {
        GitLabRunner {
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
