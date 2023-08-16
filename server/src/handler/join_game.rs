use axum::{extract, extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::option::Option;

use std::sync::Arc;

use crate::app::App;
use crate::db::common::Uuid;
use crate::db::game::Game;
use crate::db::game_player;
use crate::db::game_player::GamePlayer;
use crate::db::user::User;

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub game_id: Uuid<Game>,
    pub user_id: Uuid<User>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub game_player_id: Option<Uuid<GamePlayer>>,
}

pub async fn handler(
    State(app): State<Arc<App>>,
    extract::Json(payload): extract::Json<Request>,
) -> (StatusCode, Json<Response>) {
    let Request { game_id, user_id } = payload;
    let client = app.db_pool.get().await.unwrap();
    let res = game_player::try_join_game(&client, &game_id, &user_id).await;
    match res {
        None => (
            StatusCode::FORBIDDEN,
            Json(Response {
                game_player_id: None,
            }),
        ),
        Some(res) => (
            StatusCode::OK,
            Json(Response {
                game_player_id: Some(res.id),
            }),
        ),
    }
}
