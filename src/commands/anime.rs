use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::jikan;

#[poise::command(
    slash_command,
    rename = "get_random_anime",
    description_localized("pt-BR", "Recomendo um anime")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let response = jikan::get_random_anime().await;

    match response {
        Ok(res) => {
            ctx.send(
                CreateReply::default()
                    .embed(embeds::anime::basic(res.data))
                    .content("Aqui está uma recomendação especial para você! Espero que goste! 🥰"),
            )
            .await?;
        }
        Err(_err) => {
            println!("{:?}", _err);

            ctx.say("Desculpe... algo deu errado! Não fique triste, ok? Vou resolver isso rapidinho e trazer outra recomendação incrível! 🌸✨")
                .await?;
        }
    };

    Ok(())
}
