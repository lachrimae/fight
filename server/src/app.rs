use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    pub http_addr: String,
    pub udp_addr: String,
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

#[derive(Clone)]
pub struct App {
    pub db_pool: deadpool_postgres::Pool,
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
