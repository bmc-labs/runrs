// Copyright 2024 bmc::labs GmbH. All rights reserved.

use atmosphere::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Schema, PartialEq, Eq)]
#[table(schema = "ignored", name = "runners")]
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
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn create_delete() -> eyre::Result<()> {
        let pool = atmosphere::Pool::connect("sqlite::memory:").await.unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        let mut runner = Runner {
            id: 42,
            url: "https://gitlab.bmc-labs.com".to_owned(),
            token: "gltok-warblgarbl".to_owned(),
            description: "Knows the meaning of life".to_owned(),
            image: "alpine:latest".to_owned(),
            tag_list: "runnertest,wagarbl".to_owned(),
            run_untagged: false,
        };

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
