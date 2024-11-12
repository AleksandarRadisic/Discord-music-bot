use poise::command;
use crate::structs;

#[command(prefix_command)]
pub async fn deploy(ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    let guild_id = ctx.guild_id().expect("Error fetching guild id");
    poise::builtins::register_in_guild(ctx, &ctx.framework().options().commands,guild_id).await?;
    ctx.say("All commands have been deployed!").await?;
    Ok(())
}