
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    pub http_addr: String,
    pub amqp_addr: String,
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

pub struct App {
    pub db_pool: deadpool_postgres::Pool,
    // pub new_game_channel: lapin::Channel,
}

impl App {
    pub async fn from_cfg(cfg: &Config) -> Result<Self, ::config::ConfigError> {
        let pool = cfg
            .pg
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .unwrap();

        // let conn = Connection::connect(&cfg.amqp_addr, ConnectionProperties::default())
        //     .await
        //     .unwrap();
        // let new_game_channel = conn.create_channel().await.unwrap();
        // new_game_channel.queue_declare(
        //     "games_needing_servers",
        //     QueueDeclareOptions::default(),
        //     FieldTable::default(),
        // );

        Ok(App {
            db_pool: pool,
            // new_game_channel: new_game_channel,
        })
    }
}
