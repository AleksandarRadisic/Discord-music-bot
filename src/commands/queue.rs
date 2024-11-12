use crate::structs::{self, AuxMetadataKey};
use poise::CreateReply;
use serenity::all::CreateEmbed;
use std::string::String;

#[poise::command(slash_command)]
pub async fn queue(ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Must be used in server").await?;
            return Ok(());
        }
    };

    let manager = &ctx.data().songbird;
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.len() == 0 {
            ctx.say("Queue is empty!").await?;
            return Ok(());
        }
        let curr_queue = queue.current_queue();
        let mut song_titles: Vec<String> = vec![];
        for handle in curr_queue {
            let type_map = handle.typemap().read().await;
            let aux_metadata = match type_map.get::<AuxMetadataKey>() {
                Some(metadata) => metadata,
                None => {
                    continue;
                }
            };
            let title = match &aux_metadata.title {
                Some(title) => title.clone(),
                None => {
                    continue;
                }
            };
            song_titles.push(title);
        }
        let description = song_titles
            .iter()
            .enumerate()
            .map(|(index, title)| format!("{}. {}", index + 1, title))
            .collect::<Vec<_>>()
            .join("\n");

        let embed = CreateEmbed::new()
            .title("Current Queue")
            .description(description)
            .colour(0x00ab72)
            .timestamp(serenity::model::Timestamp::now());
        let reply_builder = CreateReply {
            embeds: vec![embed],
            reply: true,
            ..Default::default()
        };
        let _ = ctx.send(reply_builder).await;
    } else {
        ctx.say("Not in any voice channel!").await?;
    }
    Ok(())
}
