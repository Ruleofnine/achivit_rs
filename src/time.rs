use crate::{Error, Context};
use color_eyre::Result;
/// Ping the bot!
#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>, 
) -> Result<(), Error> {
    let ping = ctx.ping().await.as_millis();
    let res = format!("{}ms",ping);
    ctx.say(res).await?;
    Ok(())
}
