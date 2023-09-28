use color_eyre::Result;
use log::info;
use poise::serenity_prelude as serenity;
use poise::Event;
use crate::{Data,Error};
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
        Event::Message { new_message } => {
            info!("[{}]: {}", new_message.author.name,  new_message.content, )
        },
        Event::MessageDelete { channel_id, deleted_message_id, .. } => {
            info!("{} {} ",channel_id, deleted_message_id)
        }
        _ => {}
    }
    Ok(())
}
