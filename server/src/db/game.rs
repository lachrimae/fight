use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::fmt;
use std::str::FromStr;

use crate::db::common::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub id: String,
    pub state: GameState,
    pub created_at: String,
    pub modified_at: String,
}

impl FromRow for Game {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        Game {
            id: row.get::<usize, String>(0),
            state: GameState::from_str(row.get(1)).unwrap(),
            created_at: row.get::<usize, String>(2),
            modified_at: row.get::<usize, String>(3),
        }
    }
}
