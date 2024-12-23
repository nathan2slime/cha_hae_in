use crate::types::{Context, Error};

#[poise::command(slash_command, description_localized("pt-BR", "Pinga o bot"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();

    match ctx.data().db.clone() {
        Ok(_db) => {
            ctx.say(format!("Yay!, <@{}>! 🐾✨ Pong! 🎉", author.id))
                .await?;
        }
        Err(_e) => {
            ctx.say(format!(
                "Oh no! Parece que os hamsters estão tirando um cochilo agora, <@{}>... 💤🐹 Tente novamente mais tarde, tá bom? Eles logo estarão de volta correndo nas rodinhas! 💕",
                author.id
            ))
            .await?;
        }
    }

    Ok(())
}
