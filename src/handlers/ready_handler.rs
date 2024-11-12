use poise::serenity_prelude::{Context, EventHandler, Ready};

pub struct ReadyHandler;

#[poise::async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);
    }
}
