use color_eyre::Result;
use crate::{Error, Context};
#[poise::command(slash_command)]
pub async fn wiki(ctx: Context<'_>, query: Option<String>) -> Result<(), Error> {
    ctx.say(format!("{}",query.unwrap_or("None".to_string()))).await?;
    Ok(())
}
