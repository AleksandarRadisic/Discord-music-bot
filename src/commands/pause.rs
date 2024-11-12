use crate::structs;
use std::string::String;

#[poise::command(slash_command)]
pub async fn pause(
    ctx: structs::Context<'_>
) -> Result<(), structs::Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Must be used in server").await?;
            return Ok(());
        }
    };
    {
        let handler_lock = if let Some(handler_lock) = ctx.data().songbird.get(guild_id){
            handler_lock
        }else{
            let _ = ctx.say("Error has occoured, try again").await;
            return Ok(())
        };
        let handler = handler_lock.lock().await;
        let _ = handler.queue().pause();
    }
    let _ = ctx.say("Queue paused").await;
    Ok(())
}