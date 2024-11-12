use crate::{
    structs::{self, set_default_guild_channel},
    utils::{regex_utils::parse_query, serenity_utils::{self, add_handlers, check_msg, SongbirdState}},
};
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateMessage};
use std::{string::String, sync::Arc};

#[poise::command(slash_command)]
pub async fn play(
    ctx: structs::Context<'_>,
    #[description = "query"] query: std::string::String,
) -> Result<(), structs::Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let channel_id = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(ctx.reply("Join a voice channel first!").await);
            return Ok(());
        }
    };
    let message_channel_id = set_default_guild_channel(&ctx.data().guild_default_channel, guild_id, ctx.channel_id());
    let manager = &ctx.data().songbird;
    let songbird_state =
        serenity_utils::check_songbird_state(manager, &guild_id, &channel_id).await.unwrap();
    if songbird_state == SongbirdState::InDifferentChannel {
        check_msg(
            ctx.say("You need to be in the same voice channel as Songbird client (bot)")
                .await,
        );
        return Ok(());
    }
    if songbird_state == SongbirdState::NotInVoiceChannel {
        let _ = manager.join(guild_id, channel_id).await;
    }
    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();
    ctx.defer().await?;
    if query.contains("&list"){
        
    }
    let songs = parse_query(&query);
    if songs.len() > 1 {
        check_msg(ctx.say("Adding playlist").await);
    }
    for (_index, url) in songs.iter().enumerate() {
        let (track_handle, song) = match serenity_utils::enqueue_song(
            manager,
            url.to_string(),
            guild_id,
            data.reqwest_client.clone(),
        )
        .await
        {
            Ok((handle, song)) => (handle, song),
            Err(error_message) => {
                if songs.len() > 1 {
                    continue;
                }
                check_msg(
                    ctx.say(format!(
                        "Error queuing song: {}. Continuing...",
                        error_message
                    ))
                    .await,
                );
                continue;
            }
        };
        let create_message = CreateMessage::new();
        let create_embed = CreateEmbed::new()
            .title(format!(
                "{}",
                &song.title.clone().unwrap_or_else(|| { "".to_string() })
            ))
            .colour(0xffff00)
            .description(format!(
                "Song: **{}** added to queue",
                song.title.clone().unwrap_or_else(|| { "".to_string() })
            ))
            .thumbnail(song.thumbnail.clone().unwrap_or_else(|| {
                "https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3"
                    .to_string()
            }))
            .timestamp(serenity::model::Timestamp::now());
        if songs.len() > 1 {
            let message = create_message.add_embed(create_embed);
            let _ = ctx
                .http()
                .send_message(message_channel_id, vec![], &message)
                .await;
        } else {
            let reply_builder = CreateReply {
                embeds: vec![create_embed],
                reply: true,
                ..Default::default()
            };
            let _ = ctx.send(reply_builder).await;
        }
        add_handlers(
            &track_handle,
            &song,
            message_channel_id,
            guild_id,
            Arc::clone(&ctx.serenity_context().http),
            Arc::clone(&manager),
            Arc::clone(&ctx.data().guild_queue_loop),
            ctx.data().reqwest_client.clone(),
        )
        .await;
    }
    if songs.len() > 1 {
        check_msg(ctx.say("Playlist added").await);
    }
    Ok(())
}
