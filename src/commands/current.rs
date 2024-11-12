use crate::structs::{self, AuxMetadataKey};
use std::string::String;
use poise::CreateReply;
use serenity::all::CreateEmbed;

#[poise::command(slash_command)]
pub async fn current(ctx: structs::Context<'_>) -> Result<(), structs::Error> {
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
        let current_track = &queue.current_queue()[0];
        let type_map = current_track.typemap().read().await;

        let metadata = match type_map.get::<AuxMetadataKey>() {
            Some(metadata) => metadata,
            None => {
                ctx.say("No metadata found for the current song.").await?;
                return Ok(());
            },
        };

        let song_title = metadata.title.clone().unwrap_or("Unknown Title".to_string());

        let thumbnail_url = metadata.thumbnail.clone().unwrap_or_else(|| {
            "https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3".to_string()
        });

        let embed = CreateEmbed::new()
            .title(format!("Current Song: {}", song_title))
            .thumbnail(thumbnail_url)
            .colour(0x00ff72)
            .timestamp(serenity::model::Timestamp::now());
        let reply_builder = CreateReply{
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