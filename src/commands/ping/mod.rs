use crate::types::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();

    ctx.say(format!("pong <@{}> !", author.id)).await?;

    Ok(())
}
