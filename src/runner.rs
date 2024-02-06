// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Schema, PartialEq, Eq)]
#[table(schema = "public", name = "runners")]
pub struct Runner {
    #[sql(pk)]
    pub id: i32,
    pub url: String,
    pub token: String,
    pub description: String,
    pub image: String,
    pub tag_list: String,
    pub run_untagged: bool,
}

#[cfg(test)]
impl Runner {
    pub fn for_testing() -> Self {
        Runner {
            id: 42,
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
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn create_delete() -> eyre::Result<()> {
        let pool = atmosphere::Pool::connect("sqlite::memory:").await?;
        sqlx::migrate!().run(&pool).await?;

        let mut runner = Runner::for_testing();

        assert_eq!(Runner::find(&runner.id, &pool).await?, None);
        assert_eq!(runner.create(&pool).await?.rows_affected(), 1);
        assert_eq!(
            Runner::find(&runner.id, &pool).await?.as_ref(),
            Some(&runner)
        );
        assert_eq!(runner.delete(&pool).await?.rows_affected(), 1);
        assert_eq!(Runner::find(&runner.id, &pool).await?, None);

        Ok(())
    }
}
