use log::info;
use reqwest::Client;
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

pub fn create_http_client() -> Client {
    Client::new()
}
