use songbird::input::YoutubeDl;

use crate::types::{Context, Error};

#[poise::command(slash_command, description_localized("pt-BR", "Ouça alguma coisa <3"))]
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL do YouTube"] url: String,
) -> Result<(), Error> {
    let do_search = !url.starts_with("http");

    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();

    if let Some(handler_lock) = data.songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(data.http.clone(), url)
        } else {
            YoutubeDl::new(data.http.clone(), url)
        };

        let _ = handler.play(src.into());

        ctx.say("Hmm, está tocando agora... Espero que seja do seu gosto!")
            .await?;
    } else {
        ctx.say("Ah, eu não estou em um canal de voz ainda... por favor, me convide!")
            .await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    rename = "join",
    description_localized("pt-BR", "Entra em um canal de voz")
)]
pub async fn join_channel(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    if channel_id.is_some() {
        let manager = &ctx.data().songbird;

        let status = manager.join(guild_id, channel_id.unwrap()).await;
        ctx.say("Entrei no canal! Por favor, me diga o que quer ouvir~")
            .await?;

        match status {
            Ok(_handler) => {}
            Err(_err) => {}
        }
    } else {
        ctx.say("Eh? Você não está em um canal de voz! Como posso entrar sozinha?")
            .await?;
    }

    Ok(())
}
