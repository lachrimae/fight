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

const NEW_GAME: &str = include_str!("./game/new.sql");
const CANCEL_GAME: &str = include_str!("./game/cancel.sql");
const GET_GAME: &str = include_str!("./game/get.sql");
const GET_LOBBIED: &str = include_str!("./game/get_lobbied.sql");

impl Game {
    pub async fn new(client: &Client, initiating_user_id: &Uuid<User>) -> Self {
        let stmt = client.prepare_cached(NEW_GAME).await.unwrap();
        let row = &client
            .query_one(&stmt, &[&initiating_user_id.inner()])
            .await
            .unwrap();
        Self::from_row(row)
    }

    pub async fn cancel(self, client: &Client) -> Self {
        let stmt = client.prepare_cached(CANCEL_GAME).await.unwrap();
        let row = &client.query_one(&stmt, &[&self.id.inner()]).await.unwrap();
        Self::from_row(row)
    }

    pub async fn get(client: &Client, id: &Uuid<Game>) -> Option<Self> {
        let stmt = client.prepare_cached(GET_GAME).await.unwrap();
        let row_res = &client.query_one(&stmt, &[&id.inner()]).await;
        match row_res {
            Ok(row) => Some(Self::from_row(row)),
            Err(_) => None,
        }
    }

    pub async fn get_lobbied(client: &Client) -> Vec<Uuid<Self>> {
        let stmt = client.prepare_cached(GET_LOBBIED).await.unwrap();
        let rows = &client.query(&stmt, &[]).await.unwrap();
        rows.iter().map(|row| Uuid::new(row.get(0))).collect()
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
    use crate::db::user::User;
    #[tokio::test]
    async fn create_and_cancel() {
        let app = &crate::test::APP;
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

    async fn get_lobbied() {
        let app = &crate::test::APP;
        let client = app.db_pool.get().await.unwrap();
        let user = User::new(&client).await;
        let game = Game::new(&client, &user.id).await;
        let lobbied_games = Game::get_lobbied(&client).await;
        assert_eq!(lobbied_games.len(), 0);
        assert_eq!(lobbied_games[0], game.id);
    }
}
