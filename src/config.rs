use crate::error::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub rmq_url: String,

    #[serde(default = "default_log_level")]
    pub rust_log: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenvy::dotenv().ok();

        let config = envy::prefixed("EVENT_RELAY_").from_env::<Config>()?;

        Ok(config)
    }
}
