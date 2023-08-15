use chrono::prelude::*;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use std::option::Option;
use std::vec::Vec;

use crate::db::common::*;
use crate::db::game::{Game, GameState};
use crate::db::user::User;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct GamePlayer {
    pub id: Uuid<GamePlayer>,
    pub game_id: Uuid<Game>,
    pub user_id: Uuid<User>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl FromRow for GamePlayer {
    fn from_row(row: &tokio_postgres::row::Row) -> Self {
        GamePlayer {
            id: Uuid::<GamePlayer>::new(row.get(0)),
            game_id: Uuid::<Game>::new(row.get(1)),
            user_id: Uuid::<User>::new(row.get(2)),
            created_at: row.get(3),
            modified_at: row.get(4),
        }
    }
}

pub enum JoinResult {
    Success,
    GameFull,
}

const JOIN_GAME: &str = include_str!("./game_player/join.sql");
const LEAVE_GAME: &str = include_str!("./game_player/leave.sql");
const GET_PLAYERS: &str = include_str!("./game_player/get_players.sql");
const CANCEL_IF_ZERO_PLAYERS: &str = include_str!("./game_player/cancel_if_zero_players.sql");

pub async fn try_join_game(
    client: &Client,
    game_id: &Uuid<Game>,
    user_id: &Uuid<User>,
) -> Option<GamePlayer> {
    let stmt = client.prepare_cached(JOIN_GAME).await.unwrap();
    let rows = &client
        .query(&stmt, &[&game_id.inner(), &user_id.inner()])
        .await
        .unwrap();
    if rows.is_empty() {
        None
    } else {
        Some(GamePlayer::from_row(&rows[0]))
    }
}

pub async fn leave_game(client: &mut Client, game_id: &Uuid<Game>, user_id: &Uuid<User>) {
    let delete_player = client.prepare_cached(LEAVE_GAME).await.unwrap();
    let txn = client.transaction().await.unwrap();
    txn.query(&delete_player, &[&game_id.inner(), &user_id.inner()])
        .await
        .unwrap();
    let maybe_cancel_game = txn.prepare_cached(CANCEL_IF_ZERO_PLAYERS).await.unwrap();
    txn.query(&maybe_cancel_game, &[&game_id.inner()])
        .await
        .unwrap();
    txn.commit().await.unwrap();
}

async fn current_players(client: &Client, game_id: &Uuid<Game>) -> Vec<User> {
    let stmt = client.prepare_cached(GET_PLAYERS).await.unwrap();
    let rows = client.query(&stmt, &[&game_id.inner()]).await.unwrap();
    User::from_rows(&rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Config};
    use crate::db::game::Game;
    use crate::db::user::User;
    #[tokio::test]
    async fn test_try_join_game() {
        let app = &crate::test::APP;
        let client = app.db_pool.get().await.unwrap();
        let player = User::new(&client).await;
        let game = Game::new(&client, &player.id).await;
        let players = current_players(&client, &game.id).await;
        assert!(players.iter().map(|&p| p.id).collect::<Vec<_>>() == vec![player.id]);
    }

    #[tokio::test]
    async fn test_leave_game() {
        let app = &crate::test::APP;
        let mut client = app.db_pool.get().await.unwrap();
        let player = User::new(&client).await;
        let game = Game::new(&client, &player.id).await;
        leave_game(&mut client, &game.id, &player.id).await;
        let players = current_players(&client, &game.id).await;
        assert_eq!(players.len(), 0);
        let game = Game::get(&client, &game.id).await.unwrap();
        assert_eq!(game.state, GameState::Cancelled);
    }

    #[tokio::test]
    async fn test_second_player_joins() {
        let app = &crate::test::APP;
        let mut client = app.db_pool.get().await.unwrap();
        let player1 = User::new(&client).await;
        let player2 = User::new(&client).await;
        let game = Game::new(&client, &player1.id).await;
        let players = current_players(&client, &game.id).await;
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].id, player1.id);
        assert!(try_join_game(&client, &game.id, &player2.id)
            .await
            .is_some());
        let players = current_players(&client, &game.id).await;
        assert_eq!(players.len(), 2);
        assert_eq!(
            players.iter().map(|&p| p.id).collect::<Vec<_>>().sort(),
            vec![player1.id, player2.id].sort()
        );
    }

    #[tokio::test]
    async fn test_three_player_join_leave() {
        let app = &crate::test::APP;
        let mut client = app.db_pool.get().await.unwrap();
        let player1 = User::new(&client).await;
        let player2 = User::new(&client).await;
        let player3 = User::new(&client).await;
        let game = Game::new(&client, &player1.id).await;
        assert!(try_join_game(&client, &game.id, &player2.id)
            .await
            .is_some());
        assert!(try_join_game(&client, &game.id, &player3.id)
            .await
            .is_none());
        leave_game(&mut client, &game.id, &player1.id).await;
        assert!(try_join_game(&client, &game.id, &player3.id)
            .await
            .is_some());
        assert!(try_join_game(&client, &game.id, &player1.id)
            .await
            .is_none());
    }
}
