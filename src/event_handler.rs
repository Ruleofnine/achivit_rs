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
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            info!("Logged in as: {} ", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
