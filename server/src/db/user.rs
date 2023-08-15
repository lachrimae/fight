use crate::db::common::FromRow;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub created_at: String,
    pub modified_at: String,
}

impl User {
    async fn make_new(client: Client) -> Self {
        Self::from_row(
            &client
                .query(
                    "insert into fight.user (id, created_at, modified_at) values ('', '', '')",
                    &[],
                )
                .await
                .unwrap()[0],
        )
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
mod tests {}
