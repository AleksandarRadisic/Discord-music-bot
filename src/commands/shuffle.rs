use crate::{structs, utils::common_utils::shuffle_vecdeque_excluding_first};
use std::string::String;

#[poise::command(slash_command)]
pub async fn shuffle(ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Must be used in server").await?;
            return Ok(());
        }
    };

    let songbird = &ctx.data().songbird;
    let handler_lock = match songbird.get(guild_id) {
        Some(lock) => lock,
        None => {
            ctx.say("Not in any voice channel!").await?;
            return Ok(());
        },
    };
    {
        let handler = handler_lock.lock().await;
        handler.queue().modify_queue(|queue| {
            shuffle_vecdeque_excluding_first(queue);
        });
    }
    let _ = ctx.say("Queue shuffled").await;
    
    Ok(())
}
