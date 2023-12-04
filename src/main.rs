// #![recursion_limit="256"]
use dotenv::dotenv;
use log::info;
use lookup_df::lookup_df_character;
use poise::serenity_prelude as serenity;
use std::env;
mod dev_tools;
mod error_handler;
mod event_handler;
mod lookup_df;
mod time;
mod wiki;
use crate::event_handler::event_handler;
use error_handler::on_error;
use std::time::Instant;
mod db;
mod requests;
mod manage_users;
mod embeds;
use crate::serenity::GuildId;
use sqlx::PgPool;
mod parsing;
mod rng;
mod sheets;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    start_time: Instant,
    db_connection: PgPool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_connection = db::establish_connection().await?;
    let token = env::var("BOT_TOKEN").expect("Missing `BOT_TOKEN` env var,");
    let start_time = Instant::now();
    color_eyre::install().expect("Failed to install color_eyre");
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,serenity=error,tracing=error"),
    );

    let guild_id = GuildId(
        env::var("DEBUG_GUILD")
            .expect("Expected DEBUG_GUILD in environment")
            .parse()
            .expect("DEBUG_GUILD must be an integer"),
    );
    let commands = vec![
        wiki::wiki(),
        time::ping(),
        time::uptime(),
        time::server_time(),
        time::random_event(),
        dev_tools::list_slash_commands(),
        manage_users::register_character(),
        manage_users::delete_character(),
        lookup_df::lookup_df_character(),
    ];
    let test_commands = vec![lookup_df_character()];
    let options = poise::FrameworkOptions {
        commands,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
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
                poise::builtins::register_in_guild(ctx, &test_commands, guild_id).await?;
                let commands: Vec<String> = framework
                    .options()
                    .commands
                    .iter()
                    .map(|c| c.name.to_owned())
                    .collect();
                info!("Registered Commands: {:?}", commands);
                Ok(Data {
                    start_time,
                    db_connection,
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::privileged() | serenity::GatewayIntents::non_privileged(),
        )
        .run()
        .await
        .unwrap();
    Ok(())
}
