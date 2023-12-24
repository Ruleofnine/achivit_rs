use crate::{Context, Error};
use color_eyre::Result;
use log::info;
#[poise::command(slash_command, owners_only, guild_only)]
pub async fn list_slash_commands(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Command Executed").await?;
    let commands = ctx.http().get_global_application_commands().await?;
    info!("Global Commands");
    for command in &commands {
        info!("{}", command.name);
    }
    info!("Guild Commands");
    let commands = ctx
        .http()
        .get_guild_application_commands(*ctx.guild_id().unwrap().as_u64())
        .await?;
    for command in &commands {
        info!("{}", command.name)
    }
    Ok(())
}

#[poise::command(slash_command, owners_only, guild_only)]
pub async fn clear_slash_commands(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Command Executed").await?;
    let commands = ctx.http().get_global_application_commands().await?;
    info!("Global Commands");
    for command in &commands {
        ctx.http().delete_global_application_command(command.id.0).await?;
    }
    Ok(())
}

