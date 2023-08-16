use axum::{
    http::StatusCode,
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};

use std::sync::Arc;

mod app;
mod db;
mod handler;
mod test;

use app::{App, Config};
use db::game::Game;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GameJoinInfo {
    mac_key: String,
    game: Game,
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env().unwrap();
    let app = Arc::new(App::from_cfg(&cfg).await.unwrap());

    tracing_subscriber::fmt::init();

    let http_app = axum::Router::new()
        .route("/version", get(version))
        .route("/games", get(handler::get_lobbied_games::handler))
        .route("/games", post(make_game))
        .route("/games/:id/join", post(handler::join_game::handler))
        .with_state(app);

    axum::Server::bind(&cfg.http_addr.parse().unwrap())
        .serve(http_app.into_make_service())
        .await
        .unwrap();
}

async fn version() -> (StatusCode, String) {
    (StatusCode::OK, "0.1.0".to_string())
}

async fn make_game() -> (StatusCode, Json<GameJoinInfo>) {
    panic!()
}
