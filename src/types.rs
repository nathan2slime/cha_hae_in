use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

pub struct Data {
    pub config: AppConfig,
    pub db: Result<DatabaseConnection, bool>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
