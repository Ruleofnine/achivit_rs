use crate::db::insert_guild;
use crate::{Data, Error};
use color_eyre::Result;
use log::info;
use poise::serenity_prelude as serenity;
use poise::Event;
#[allow(unused)]
pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            let prefix =                 framework
                    .options()
                    .prefix_options
                    .prefix
                    .as_deref().expect("No Prefix");
            info!(
                "Logged in as: {} with prefix: {}",
                data_about_bot.user.name,
                prefix
            );
        }
        Event::GuildCreate { guild, is_new } => {
            info!("Initilazing guild {}:{}", guild.id.0, guild.name);
            let pool = &data.db_connection;
            insert_guild(pool, guild).await;
        }
        _ => {}
    }
    Ok(())
}
