use crate::db::common::FromRow;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub created_at: String,
    pub modified_at: String,
}

impl User {
    async fn make_new(client: &Client) -> Self {
        let stmt = client
            .prepare_cached(
                "insert into fight.user default values returning (id, created_at, modified_at)",
            )
            .await
            .unwrap();
        let row = &client.query(&stmt, &[]).await.unwrap()[0];
        Self::from_row(&row)
    }
}

impl FromRow for User {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        User {
            id: row.get::<usize, String>(0),
            created_at: row.get::<usize, String>(1),
            modified_at: row.get::<usize, String>(2),
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
        let user = super::User::make_new(&client).await;
        println!("{:#?}", user);
    }
}
