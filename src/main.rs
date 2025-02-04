mod commands;
mod config;
mod embeds;
mod services;
mod types;
mod utils;

use crate::commands::commands;
use crate::config::load_config;

use dotenv::dotenv;
use serenity::all::{ClientBuilder, GatewayIntents};
use songbird::Songbird;
use types::Data;
use utils::create_http_client;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let config = load_config();
    let intents = GatewayIntents::non_privileged();
    let discord_token = config.discord_token.clone();
    let voice_manager = Songbird::serenity();

    let my_voice_manager = voice_manager.clone();

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
                    http: create_http_client(),
                    songbird: my_voice_manager,
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, intents)
        .event_handler(utils::Handler)
        .framework(framework)
        .voice_manager_arc(voice_manager)
        .await;

    tokio::spawn(async move {
        client.unwrap().start().await.unwrap();
    });

    let _signal_err = tokio::signal::ctrl_c().await;

    println!("received ctrl-c");
}
