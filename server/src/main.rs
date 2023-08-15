use axum::{
    http::StatusCode,
    routing::{delete, get, post},
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

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

    launch_udp(cfg.udp_addr.parse().unwrap()).await;

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
