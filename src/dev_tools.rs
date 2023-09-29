use color_eyre::Result;
use log::info;
use crate::{Context,Error};
#[poise::command(slash_command,owners_only,guild_only)]
pub async fn list_slash_commands(ctx: Context<'_>) -> Result<(),Error>{
    ctx.say("Command Executed").await?;
    let  commands = ctx.http().get_global_application_commands().await?;
    for command in &commands {
        info!("{:?}",command);
    }
    let  commands = ctx.http().get_guild_application_commands(*ctx.guild_id().unwrap().as_u64()).await?;
    for command in &commands {
        info!("{:?}",command)
    } 
    Ok(())
}

pub async fn get_bot_avatar(ctx: &Context<'_>) -> Result<String,Error> {
    let avatar_url =ctx.serenity_context().cache.current_user().avatar_url();
    match avatar_url {
        Some(url) => Ok(url),
        None => Ok("https://cdn.drawception.com/images/panels/2016/12-10/Q4Zcfan1X5-6.png".to_string())
    }
}
