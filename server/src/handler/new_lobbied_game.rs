use crate::app::App;
use crate::db::common::Uuid;
use crate::db::game::Game;

use axum::{extract::State, http::StatusCode, Json};
use std::str::FromStr;
use std::sync::Arc;

pub async fn handler(State(app): State<Arc<App>>, user_id: &str) -> (StatusCode, Json<Uuid<Game>>) {
    let client = app.db_pool.get().await.unwrap();
    let game = Game::new(&client, &Uuid::new(uuid::Uuid::from_str(user_id).unwrap())).await;
    (StatusCode::OK, Json(game.id))
}
