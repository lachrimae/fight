use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub amqp_addr: String,
    pub player_ids: String,
    pub game_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ::config::ConfigError> {
        let mut cfg: Config = ::config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()?;
        Ok(cfg)
    }
}
