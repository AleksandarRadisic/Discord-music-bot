use crate::structs::{self, set_guild_loop};
use std::string::String;

#[poise::command(slash_command)]
pub async fn loop_queue(
    ctx: structs::Context<'_>,
    #[description = "true or false"] do_loop: bool,
) -> Result<(), structs::Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("Must be used in server").await?;
            return Ok(());
        }
    };
    let loop_map = &ctx.data().guild_queue_loop;
    set_guild_loop(loop_map, guild_id, do_loop);
    if !do_loop {
        let _ = ctx.say("Loop disabled").await;
    } else {
        let _ = ctx.say("Loop enabled").await;
    }
    Ok(())
}
