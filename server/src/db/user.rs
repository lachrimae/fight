use crate::db::common::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub created_at: String,
    pub modified_at: String,
}

impl FromRow for User {
    fn from_row(row: tokio_postgres::row::Row) -> Self {
        User {
            id: row.get::<usize, String>(0),
            created_at: row.get::<usize, String>(1),
            modified_at: row.get::<usize, String>(2),
        }
    }
}
