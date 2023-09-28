use df::wiki::wiki;
use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;
use std::env;
mod df;
mod error_handler;
mod event_handler;
mod time;
use crate::event_handler::event_handler;
use error_handler::on_error;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}
#[tokio::main]
async fn main() {
    dotenv().ok();
    color_eyre::install().expect("Failed to install color_eyre");
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,serenity=error,tracing=error"),
    );
    let options = poise::FrameworkOptions {
        commands : vec![
            wiki(),
            time::ping(),
        ],
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                info!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                info!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(env::var("BOT_TOKEN").expect("Missing `BOT_TOKEN` env var,"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let commands: Vec<String> = framework.options().commands.iter().map(|c| c.name.to_owned()).collect();
                info!("Registered Commands: {:?}",commands);
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::privileged() | serenity::GatewayIntents::non_privileged(),
        )
        .run()
        .await
        .unwrap();
}

