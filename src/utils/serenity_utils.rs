use std::{collections::HashMap, sync::{Arc, RwLock}, time::Duration};

use reqwest::Client;
use serenity::all::{ChannelId, GuildId, Http};
use songbird::{
    input::{AuxMetadata, YoutubeDl},
    tracks::TrackHandle,
    Songbird, TrackEvent,
};
use tracing::{error, warn};

use crate::{
    handlers::{song_end_handler::SongEndHandler, song_start_handler::SongStartNotifier},
    structs::{AuxMetadataKey, ChannelIdKey, GuildIdKey, HttpKey},
};

#[derive(PartialEq, Debug)]
pub enum SongbirdState {
    NotInVoiceChannel,
    InSameChannel,
    InDifferentChannel,
}

pub async fn play_audio_input(file_path: &String, manager: &Arc<Songbird>, guild_id: GuildId) {
    let input = songbird::input::File::new(file_path.to_owned());
    if let Some(handle_lock) = manager.get(guild_id) {
        let mut handler = handle_lock.lock().await;
        let _ = handler.play_input(input.into());
    }
}

pub async fn check_songbird_state(
    songbird: &Arc<Songbird>,
    guild_id: &GuildId,
    channel_id: &ChannelId,
) -> Result<SongbirdState, Box<dyn std::error::Error>> {
    match songbird.get(*guild_id) {
        Some(handler_lock) => {
            let handler = handler_lock.lock().await;
            match handler.current_channel() {
                Some(current_channel_id) if current_channel_id == (*channel_id).into() => {
                    Ok(SongbirdState::InSameChannel)
                }
                Some(_) => Ok(SongbirdState::InDifferentChannel),
                None => Ok(SongbirdState::NotInVoiceChannel),
            }
        }
        None => Ok(SongbirdState::NotInVoiceChannel),
    }
}

pub async fn enqueue_song(
    songbird: &Arc<Songbird>,
    query: String,
    guild_id: GuildId,
    client: Client,
) -> Result<(TrackHandle, AuxMetadata), String> {
    if let Some(handler_lock) = songbird.get(guild_id) {
        let do_search = !query.starts_with("http");
        let mut src = if do_search {
            YoutubeDl::new_search(client, query.clone())
        } else {
            YoutubeDl::new(client, query.clone())
        };
        let src_metadata = src.search(Some(1)).await.map_err(|err| {
            let error_msg = format!("Error searching for song '{}': {}", query, err);
            error!("{}", error_msg);
            error_msg
        })?;
        let song = src_metadata.first().cloned().ok_or_else(|| {
            let error_msg = format!("No results found for query '{}'", query);
            error!("{}", error_msg);
            error_msg
        })?;
        let mut handler = handler_lock.lock().await;
        let track_handle = handler.enqueue_with_preload(
            src.into(),
            song.duration
                .filter(|&dur| dur > Duration::from_secs(10))
                .map(|dur| dur - Duration::from_secs(10)),
        );
        Ok((track_handle, song))
    } else {
        let error_msg = format!("No Songbird handler found for guild '{}'", guild_id);
        error!("{}", error_msg);
        Err(error_msg)
    }
}

pub async fn add_handlers(
    track_handle: &TrackHandle,
    aux_metadata: &AuxMetadata,
    channel_id: ChannelId,
    guild_id: GuildId,
    http: Arc<Http>,
    songbird: Arc<Songbird>,
    guild_queue_loop: Arc<RwLock<HashMap<GuildId, bool>>>,
    reqwest_client: Client,
) {
    let mut type_map = track_handle.typemap().write().await;
    type_map.insert::<AuxMetadataKey>(aux_metadata.clone());
    type_map.insert::<ChannelIdKey>(channel_id);
    type_map.insert::<GuildIdKey>(guild_id);
    type_map.insert::<HttpKey>(Arc::clone(&http));
    {
        let handler_lock = songbird.get(guild_id).unwrap();
        let handle = handler_lock.lock().await;
        if handle.queue().len() == 1 {
            let _ = track_handle.add_event(TrackEvent::Playable.into(), SongStartNotifier);
        }
    }
    let _ = track_handle.add_event(TrackEvent::Play.into(), SongStartNotifier);
    let _ = track_handle.add_event(
        TrackEvent::End.into(),
        SongEndHandler {
            songbird: Arc::clone(&songbird),
            guild_loop_map_arc: Arc::clone(&guild_queue_loop),
            guild_id,
            client: reqwest_client.clone(),
            url: aux_metadata.source_url.clone().unwrap(),
            channel_id,
            http_arc: Arc::clone(&http),
        },
    );
}

pub fn check_msg<T>(result: serenity::Result<T>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
    }
}
