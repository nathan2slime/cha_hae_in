use log::info;
use poise::CreateReply;

use crate::types::{Context, Error};

use crate::embeds;
use crate::services::nasa::get_astronomy_picture_day;

#[poise::command(
    slash_command,
    description_localized("pt-BR", "Mostra a imagem do dia da NASA")
)]
pub async fn apod(ctx: Context<'_>) -> Result<(), Error> {
    let nasa_api_key = ctx.data().config.nasa_api_key.clone();
    let res = get_astronomy_picture_day(nasa_api_key).await;

    match res {
        Ok(data) => {
            ctx.send(CreateReply::default().embed(embeds::nasa::apod(data)))
                .await?;
        }
        Err(_err) => {
            info!("{:?}", _err);
            ctx.say("Desculpe... algo deu errado. Vamos tentar de novo depois, ok? 🌸")
                .await?;
        }
    };

    Ok(())
}
