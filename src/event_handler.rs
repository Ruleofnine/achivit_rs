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
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("Logged in as: {} ", data_about_bot.user.name);
        },
            Event::GuildCreate { guild, is_new }=>{
            info!("Initilazing guild {}:{}",guild.id.0,guild.name);
            let pool = &data.db_connection;
            insert_guild(pool, guild).await;


        },
        _ => {}
    }
    Ok(())
}
