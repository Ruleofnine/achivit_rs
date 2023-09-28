use color_eyre::Result;
use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;
use std::env;
mod df;
mod event_handler;
use crate::event_handler::event_handler;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main()  {
    dotenv().ok();
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,serenity=error,tracing=error"),
    );
 let options = poise::FrameworkOptions {
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            env::var("BOT_TOKEN")
                .expect("Missing `BOT_TOKEN` env var, see README for more information."),
        )
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}

