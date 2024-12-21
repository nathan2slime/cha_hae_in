mod config;
mod types;
mod commands;

use serenity::all::ClientBuilder;
use serenity::prelude::GatewayIntents;
use crate::config::load_config;
use crate::commands::commands;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let config = load_config();
    let intents = GatewayIntents::non_privileged();
    let bot_config = config.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(bot_config)
            })
        })
        .build();

    let client = ClientBuilder::new(&config.discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
