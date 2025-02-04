use crate::types::{Context, Error};

#[poise::command(slash_command, description_localized("pt-BR", "Pinga o bot"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();

    ctx.say(format!("Yay!, <@{}>! 🐾✨ Pong! 🎉", author.id))
        .await?;

    Ok(())
}
