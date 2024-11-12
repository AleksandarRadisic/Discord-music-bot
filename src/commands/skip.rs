use crate::structs;
use std::string::String;

#[poise::command(slash_command)]
pub async fn skip(
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
        let _ = queue.skip();
        ctx.say("Skipped current song!").await?;
    }
    else{
        ctx.say("Not in any voice channel!").await?;
    }
    Ok(())
}