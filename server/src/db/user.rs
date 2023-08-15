use crate::db::common::FromRow;
use chrono::prelude::*;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

const NEW_USER: &'static str = include_str!("./user/new.sql");

impl User {
    pub async fn new(client: &Client) -> Self {
        let stmt = client.prepare_cached(NEW_USER).await.unwrap();
        let row = &client.query(&stmt, &[]).await.unwrap()[0];
        Self::from_row(&row)
    }
}

impl FromRow for User {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        User {
            id: row.get(0),
            created_at: row.get(1),
            modified_at: row.get(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Config};
    #[tokio::test]
    async fn insert_and_get() {
        let cfg = Config::from_env().unwrap();
        let app = App::from_cfg(&cfg).unwrap();
        let client = app.db_pool.get().await.unwrap();
        let user = super::User::new(&client).await;
    }
}
