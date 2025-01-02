#![allow(clippy::pedantic)]
use crate::serenity::GuildId;
use achivit_rs::db::establish_connection;
use achivit_rs::error_handler::on_error;
use achivit_rs::event_handler::event_handler;
use achivit_rs::{get_command_list, print_banner, Data};
use color_eyre::Result;
use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;
use std::env;
use std::time::Instant;
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let random_banner = env::var("RANDOM_BANNER_COLOR")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false);
    print_banner(random_banner);
    let token = env::var("BOT_TOKEN").expect("`BOT_TOKEN` not in .env file");
    let start_time = Instant::now();
    color_eyre::install().expect("Failed to install color_eyre");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let db_connection = establish_connection().await?;
    info!("Logining into Discord...");
    let _guild_id = GuildId(
        env::var("DEBUG_GUILD")
            .expect("Expected `DEBUG_GUILD` in environment")
            .parse()
            .expect("DEBUG_GUILD must be an integer"),
    );
    let super_user_str = env::var("SUPERUSERS").expect("`SUPERUSERS` not found in .env file");
    let prefix = env::var("COMMAND_PREFIX").expect("`COMMAND_PREFIX` not found in .env file");
    let super_users: Vec<u64> = super_user_str
        .split(',')
        .map(|s| {
            s.parse::<u64>()
                .expect("Failed to parse number to discord id")
        })
        .collect();

    let options = poise::FrameworkOptions {
        commands: get_command_list(),
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        on_error: |error| Box::pin(on_error(error)),
        post_command: |ctx| {
            Box::pin(async move {
                info!(
                    "{} Executed command: {}",
                    ctx.author().name,
                    ctx.command().qualified_name
                )
            })
        },
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(prefix),
            ..Default::default()
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(&token)
        .setup(move |ctx, _ready, framework| {
            info!("Setting up Poise Framework");
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::new(start_time, db_connection, super_users))
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
