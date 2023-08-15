use crate::app::App;
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use std::vec::Vec;

use crate::db::common::Uuid;
use crate::db::game::Game;

pub async fn handler(State(app): State<Arc<App>>) -> (StatusCode, Json<Vec<Uuid<Game>>>) {
    let client = app.db_pool.get().await.unwrap();
    let games = Game::get_lobbied(&client).await;
    (StatusCode::OK, Json(games))
}
