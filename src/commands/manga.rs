use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::jikan;

#[poise::command(
    slash_command,
    rename = "get_random_manga",
    description_localized("pt-BR", "Recomendo um mangá")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let response = jikan::get_random_manga().await;

    match response {
        Ok(res) => {
            ctx.send(
                CreateReply::default()
                    .embed(embeds::manga::basic(res.data))
                    .content("Espero que goste desse mangá 🥰"),
            )
            .await?;
        }
        Err(_err) => {
            println!("{:?}", _err);

            ctx.say("Desculpe... algo deu errado").await?;
        }
    };

    Ok(())
}
