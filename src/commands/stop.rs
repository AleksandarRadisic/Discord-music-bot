use crate::structs::{self, set_guild_loop};
use std::string::String;

#[poise::command(slash_command)]
pub async fn stop(
    ctx: structs::Context<'_>,
) -> Result<(), structs::Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Must be used in server").await?;
            return Ok(());
        },
    };

    let manager = &ctx.data().songbird;
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.len() == 0{
            ctx.say("Queue is empty!").await?;
            return Ok(());
        }
        set_guild_loop(&ctx.data().guild_queue_loop, guild_id, false);
        queue.stop();
        ctx.say("Queue stopped!").await?;
    }
    else{
        ctx.say("Not in any voice channel!").await?;
    }
    Ok(())
}
