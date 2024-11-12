use serenity::all::{CreateEmbed, CreateMessage};
use songbird::{Event, EventContext, EventHandler};
use tracing::error;

use crate::structs::{AuxMetadataKey, ChannelIdKey, HttpKey};

pub struct SongStartNotifier;

#[poise::async_trait]
impl EventHandler for SongStartNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track) = ctx {
            for (_track_state, track_handle) in *track {
                let type_map = track_handle.typemap().read().await;
                let metadata = match type_map.get::<AuxMetadataKey>() {
                    Some(metadata) => metadata,
                    None => {
                        error!("No metadata found");
                        return None;
                    }
                };
                let http = match type_map.get::<HttpKey>() {
                    Some(http) => http,
                    None => {
                        error!("No http found");
                        return None;
                    }
                };
                let channel_id = match type_map.get::<ChannelIdKey>() {
                    Some(chan) => chan,
                    None => {
                        error!("No channel id found");
                        return None;
                    }
                };
                let song_title = match metadata.title.clone(){
                    Some(title) => title,
                    None => {
                        error!("No song title found");
                        return None;
                    }
                };
                let embed =
                    CreateEmbed::new()
                        .title(format!(":notes: {}", song_title))
                        .colour(0x006fff)
                        .description(format!("Playing song: **{}**", song_title))
                        .thumbnail(metadata.thumbnail.clone().unwrap_or_else(|| {
                            "https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3".to_string()
                        }))
                        .timestamp(serenity::model::Timestamp::now());

                let message = CreateMessage::new().add_embed(embed);
                let _ = http.send_message(*channel_id, vec![], &message).await;
            }
        }
        None
    }
}
