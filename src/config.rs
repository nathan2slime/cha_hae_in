use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub discord_token: String,
}

pub fn load_config() -> AppConfig {
    let c = Config::builder()
        .add_source(Environment::default().prefix("CHA").separator("__"))
        .build()
        .expect("cannot build config");

    c.try_deserialize().expect("cannot deserialize config")
}
