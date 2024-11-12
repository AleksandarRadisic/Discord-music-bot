use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use reqwest::Client;
use serenity::all::{ChannelId, GuildId, Http};
use songbird::{Event, EventContext, EventHandler, Songbird};

use crate::{structs::is_loop_enabled, utils::serenity_utils::{self, add_handlers}};

pub struct SongEndHandler {
    pub songbird: Arc<Songbird>,
    pub guild_loop_map_arc: Arc<RwLock<HashMap<GuildId, bool>>>,
    pub guild_id: GuildId,
    pub client: Client,
    pub url: String,
    pub channel_id: ChannelId,
    pub http_arc: Arc<Http>
}

#[poise::async_trait]
impl EventHandler for SongEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        if !is_loop_enabled(&self.guild_loop_map_arc, self.guild_id) {
            return None;
        }
        let (track_handle, song) = match serenity_utils::enqueue_song(
            &self.songbird,
            self.url.clone(),
            self.guild_id,
            self.client.clone(),
        )
        .await
        {
            Ok((handle, song)) => (handle, song),
            Err(_) => {
                return None;
            }
        };
        add_handlers(
            &track_handle,
            &song,
            self.channel_id,
            self.guild_id,
            Arc::clone(&self.http_arc),
            Arc::clone(&self.songbird),
            Arc::clone(&self.guild_loop_map_arc),
            self.client.clone(),
        )
        .await;
        None
    }
}
