use chrono::prelude::*;
use deadpool_postgres::Client;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::db::common::*;
use crate::db::user::User;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToSql, FromSql, PartialEq, Eq)]
pub enum GameState {
    Lobbied,
    Started,
    Completed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: Uuid<Game>,
    pub state: GameState,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

const NEW_GAME: &'static str = include_str!("./game/new.sql");
const CANCEL_GAME: &'static str = include_str!("./game/cancel.sql");
const GET_GAME: &'static str = include_str!("./game/get.sql");

impl Game {
    pub async fn new(client: &Client, initiating_user_id: &Uuid<User>) -> Self {
        let stmt = client.prepare_cached(NEW_GAME).await.unwrap();
        let row = &client
            .query_one(&stmt, &[&initiating_user_id.inner()])
            .await
            .unwrap();
        Self::from_row(&row)
    }

    async fn cancel(self, client: &Client) -> Self {
        let stmt = client.prepare_cached(CANCEL_GAME).await.unwrap();
        let row = &client.query_one(&stmt, &[&self.id.inner()]).await.unwrap();
        Self::from_row(&row)
    }

    async fn get(client: &Client, id: &Uuid<Game>) -> Option<Self> {
        let stmt = client.prepare_cached(GET_GAME).await.unwrap();
        let row_res = &client.query_one(&stmt, &[&id.inner()]).await;
        match row_res {
            Ok(row) => Some(Self::from_row(&row)),
            Err(_) => None,
        }
    }
}

impl FromRow for Game {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        Game {
            id: Uuid::new(row.get(0)),
            state: row.get(1),
            created_at: row.get(2),
            modified_at: row.get(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Config};
    use crate::db::user::User;
    #[tokio::test]
    async fn create_and_cancel() {
        let cfg = Config::from_env().unwrap();
        let app = App::from_cfg(&cfg).unwrap();
        let client = app.db_pool.get().await.unwrap();
        let user = User::new(&client).await;
        let mut game = Game::new(&client, &user.id).await;
        assert_eq!(game.state, GameState::Lobbied);
        let id = game.id;
        game = Game::get(&client, &id).await.unwrap();
        assert_eq!(game.state, GameState::Lobbied);
        assert_eq!(game.id, id);
        game = game.cancel(&client).await;
        assert_eq!(game.state, GameState::Cancelled);
        assert_eq!(game.id, id);
    }
}
