use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use deadpool_postgres;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio_postgres::NoTls;

#[derive(Serialize)]
enum GameState {
    Lobbied,
    Started,
    Completed,
    Cancelled,
}

#[derive(Serialize)]
struct Game {
    id: String,
    state: GameState,
}

#[derive(Serialize)]
struct GameJoinInfo {
    mac_key: String,
    game: Game,
}

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
    http_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ::config::ConfigError> {
        let mut cfg: Config = ::config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()?;
        cfg.pg.manager = Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });
        Ok(cfg)
    }
}

struct App {
    db_pool: deadpool_postgres::Pool,
}

impl App {
    pub fn from_cfg(cfg: &Config) -> Result<Self, ::config::ConfigError> {
        let pool = cfg
            .pg
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .unwrap();
        Ok(App { db_pool: pool })
    }
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env().unwrap();
    let app = App::from_cfg(&cfg);

    tracing_subscriber::fmt::init();
    let app = axum::Router::new()
        .route("/version", get(version))
        .route("/games", get(get_games))
        .route("/games", post(make_game))
        .route("/games/:id", delete(cancel_game))
        .route("/games/:id/join", post(join_game));

    axum::Server::bind(&cfg.http_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn version() {}

async fn get_games() {}

async fn make_game() {}

async fn cancel_game() {}

async fn join_game() {}
