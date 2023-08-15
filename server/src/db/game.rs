use chrono::prelude::*;
use deadpool_postgres::Client;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use crate::db::common::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize, ToSql, FromSql)]
pub enum GameState {
    Lobbied,
    Started,
    Completed,
    Cancelled,
}

impl FromStr for GameState {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lobbied" => Ok(GameState::Lobbied),
            "Started" => Ok(GameState::Started),
            "Completed" => Ok(GameState::Completed),
            "Cancelled" => Ok(GameState::Cancelled),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not parse GameState {}", s),
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: Uuid,
    pub state: GameState,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

const NEW_GAME: &'static str = include_str!("./game/new.sql");
const CANCEL_GAME: &'static str = include_str!("./game/cancel.sql");

impl Game {
    async fn new(client: &Client) -> Self {
        let stmt = client.prepare_cached(NEW_GAME).await.unwrap();
        let row = &client.query(&stmt, &[]).await.unwrap()[0];
        Self::from_row(&row)
    }
}

impl FromRow for Game {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        Game {
            id: row.get(0),
            state: row.get(1),
            created_at: row.get(2),
            modified_at: row.get(3),
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
        let game = super::Game::new(&client).await;
    }
}
