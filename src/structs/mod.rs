use std::{collections::HashMap, sync::{Arc, RwLock}};

use reqwest::Client;
use serenity::{all::{ChannelId, GuildId, Http}, prelude::TypeMapKey};
use songbird::input::AuxMetadata;

use crate::config;

pub struct Data {
    pub guild_queue_loop: Arc<RwLock<HashMap<GuildId, bool>>>,
    pub guild_default_channel: Arc<RwLock<HashMap<GuildId, ChannelId>>>,
    pub config: Arc<config::Config>,
    pub reqwest_client: Client,
    pub songbird: Arc<songbird::Songbird>
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn set_guild_loop(arc_guild_queue_loop: &Arc<RwLock<HashMap<GuildId, bool>>>, guild_id: GuildId, do_loop: bool){
    let mut guild_queue_loop = arc_guild_queue_loop.write().unwrap();
    if let Some(guild_loop) = guild_queue_loop.get_mut(&guild_id){
        *guild_loop = do_loop;
    }else{
        guild_queue_loop.insert(guild_id, do_loop);
    }
}

pub fn is_loop_enabled(arc_guild_queue_loop: &Arc<RwLock<HashMap<GuildId, bool>>>, guild_id: GuildId) -> bool {
    let lock_map = arc_guild_queue_loop.read().unwrap();
    if let Some(guild_loop) = lock_map.get(&guild_id){
        return *guild_loop;
    }
    false
}

pub fn set_default_guild_channel(
    arc_guild_default_channel: &Arc<RwLock<HashMap<GuildId, ChannelId>>>, 
    guild_id: GuildId, 
    channel_id: ChannelId,
) -> ChannelId {
    let mut map = arc_guild_default_channel.write().unwrap();
    *map.entry(guild_id).or_insert(channel_id)
}


impl TypeMapKey for Data {
    type Value = Data;
}

impl Clone for Data {
    fn clone(&self) -> Self {
        Data {
            guild_queue_loop: Arc::clone(&self.guild_queue_loop),
            guild_default_channel: Arc::clone(&self.guild_default_channel),
            config: Arc::clone(&self.config),
            reqwest_client: self.reqwest_client.clone(),
            songbird: Arc::clone(&self.songbird),
        }
    }
}

pub struct AuxMetadataKey;

impl TypeMapKey for AuxMetadataKey {
    type Value = AuxMetadata;
}

pub struct ChannelIdKey;

impl TypeMapKey for ChannelIdKey{
    type Value = ChannelId;
}

pub struct GuildIdKey;

impl TypeMapKey for GuildIdKey{
    type Value = GuildId;
}

pub struct HttpKey;

impl TypeMapKey for HttpKey{
    type Value = Arc<Http>;
}