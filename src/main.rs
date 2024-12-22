mod commands;
mod config;
mod database;
mod embeds;
mod services;
mod types;

use crate::commands::commands;
use crate::config::load_config;
use serenity::all::ClientBuilder;
use serenity::prelude::GatewayIntents;

use dotenv::dotenv;
use types::Data;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let config = load_config();
    let intents = GatewayIntents::non_privileged();
    let discord_token = config.discord_token.clone();

    let db = database::connection::create(&config).await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config,
                    db: db.clone(),
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
