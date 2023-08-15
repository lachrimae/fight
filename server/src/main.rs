use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, post},
    Json,
};
use deadpool_postgres;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::vec::Vec;
use tokio::net::UdpSocket;

mod app;
mod db;

use app::{App, Config};
use db::common::FromRow;
use db::game::Game;
use db::user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GameJoinInfo {
    mac_key: String,
    game: Game,
}

async fn launch_udp(udp_socket: SocketAddr) {
    let socket = UdpSocket::bind(udp_socket).await.unwrap();
    let mut buf = vec![0; 1024];
    loop {
        let (size, peer) = socket.recv_from(&mut buf).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env().unwrap();
    let app = Arc::new(App::from_cfg(&cfg).unwrap());

    tracing_subscriber::fmt::init();

    launch_udp(cfg.udp_addr.parse().unwrap());

    let http_app = axum::Router::new()
        .route("/version", get(version))
        .route("/games", get(get_games))
        .route("/games", post(make_game))
        .route("/games/:id", delete(cancel_game))
        .route("/games/:id/join", post(join_game))
        .with_state(app);

    axum::Server::bind(&cfg.http_addr.parse().unwrap())
        .serve(http_app.into_make_service())
        .await;
}

async fn version() -> (StatusCode, String) {
    (StatusCode::OK, "0.1.0".to_string())
}

async fn get_games(State(app): State<Arc<App>>) -> (StatusCode, Json<Vec<Game>>) {
    let client = app.db_pool.get().await.unwrap();
    let stmt = client
        .prepare_cached("select * from fight.game g where g.state == 'Lobbied'")
        .await
        .unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    (StatusCode::OK, Json(Game::from_rows(&rows)))
}

async fn make_game() -> (StatusCode, Json<GameJoinInfo>) {
    panic!()
}

async fn cancel_game() {}

async fn join_game() {}
