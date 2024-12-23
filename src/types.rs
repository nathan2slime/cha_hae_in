use std::sync::Arc;

use reqwest::Client;
use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

pub struct Data {
    pub config: AppConfig,
    pub http: Client,
    pub songbird: Arc<songbird::Songbird>,
    pub db: Result<DatabaseConnection, bool>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
