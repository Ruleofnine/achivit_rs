use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;
use std::env;
mod df;
mod error_handler;
mod event_handler;
mod time;
mod dev_tools;
use crate::event_handler::event_handler;
use error_handler::on_error;
use std::time::Instant;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {start_time:Instant}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Missing `BOT_TOKEN` env var,");
    let start_time = Instant::now();
    color_eyre::install().expect("Failed to install color_eyre");
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,serenity=error,tracing=error"),
    );
    let options = poise::FrameworkOptions {
        commands : vec![
            df::wiki::wiki(),
            time::ping(),
            time::uptime(),
            time::server_time(),
            time::random_event(),
            dev_tools::list_slash_commands(),
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
        .token(&token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let commands: Vec<String> = framework.options().commands.iter().map(|c| c.name.to_owned()).collect();
                info!("Registered Commands: {:?}",commands);
                Ok(Data {start_time})
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

