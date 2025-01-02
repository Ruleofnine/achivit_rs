use crate::{Context, Error};
use color_eyre::Result;
use log::info;
/// super_users have access to certain features across the bot. This concpet 
pub async fn is_superuser_check(ctx: Context<'_>)->Result<bool,Error>{
    Ok(ctx.data().super_users.contains(&ctx.author().id.0))

}
#[poise::command(slash_command, owners_only, guild_only,default_member_permissions = "ADMINISTRATOR")]
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

#[poise::command(slash_command, owners_only, guild_only,default_member_permissions = "ADMINISTRATOR")]
pub async fn clear_guild_slash_commands(ctx: Context<'_>,guild_id:u64) -> Result<(), Error> {
    info!("Guild Commands");
    let commands = ctx
        .http()
        .get_guild_application_commands(guild_id)
        .await?;
    for command in &commands {
        ctx.http().delete_guild_application_command(guild_id,command.id.0).await?;
        info!("Deleted :{}", command.name)
    }
    ctx.say("Deleted guild slash commands Executed").await?;
    Ok(())
}

