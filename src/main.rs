pub mod commands;
pub mod config;
pub mod handlers;
pub mod structs;
pub mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use ::serenity::all::{ChannelId, GuildId};
use commands::{current, deployment, loop_queue, pause, play, queue, resume, shuffle, skip, stop};
use config::load_config;
use handlers::ready_handler::ReadyHandler;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use tracing::error;

#[tokio::main]
async fn main() {
    let config = load_config();
    let token = config.token.clone();
    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_VOICE_STATES
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        commands: vec![
            deployment::deploy(),
            play::play(),
            stop::stop(),
            skip::skip(),
            queue::queue(),
            current::current(),
            loop_queue::loop_queue(),
            shuffle::shuffle(),
            pause::pause(),
            resume::resume(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(String::from(config.prefix.clone())),
            ..Default::default()
        },
        ..Default::default()
    };
    let manager = songbird::Songbird::serenity();
    let manager_clone = Arc::clone(&manager);
    let config_clone = Arc::new(config.clone());
    let queue_loop_map: RwLock<HashMap<GuildId, bool>> = RwLock::new(HashMap::new());
    let arc_queue_loop = Arc::new(queue_loop_map);
    let guild_default_channel: RwLock<HashMap<GuildId, ChannelId>> = RwLock::new(HashMap::new());
    let arc_guild_default_channel = Arc::new(guild_default_channel);

    let framework = poise::Framework::new(options, move |ctx, _, _| {
        let config_clone = Arc::clone(&config_clone);
        let manager_clone = Arc::clone(&manager_clone);
        let arc_queue_loop = Arc::clone(&arc_queue_loop);

        Box::pin(async move {
            ctx.set_presence(None, serenity::OnlineStatus::Online);

            let data_struct = structs::Data {
                guild_queue_loop: arc_queue_loop,
                guild_default_channel: arc_guild_default_channel,
                config: config_clone,
                reqwest_client: Client::new(),
                songbird: manager_clone,
            };

            let mut data = ctx.data.write().await;
            data.insert::<structs::Data>(data_struct.clone());

            Ok(data_struct)
        })
    });

    let mut client = serenity::Client::builder(token, intents)
        .voice_manager_arc(manager)
        .event_handler(ReadyHandler)
        .framework(framework)
        .await
        .expect("Err creating client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    };
}
