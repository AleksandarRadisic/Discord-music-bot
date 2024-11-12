use std::string::String;

use crate::structs;

#[poise::command(slash_command)]
pub async fn resume(
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
        let _ = handler.queue().resume();
    }
    let _ = ctx.say("Queue resumed").await;
    Ok(())
}