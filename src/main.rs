#![allow(clippy::pedantic)]
use crate::serenity::GuildId;
use achivit_rs::db::establish_connection;
use achivit_rs::error_handler::on_error;
use achivit_rs::event_handler::event_handler;
use achivit_rs::{print_banner, Data};
use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;
use std::env;
use std::time::Instant;
use color_eyre::Result;
#[tokio::main]
async fn main() -> Result<()> {
    print_banner();
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Missing `BOT_TOKEN` env var,");
    let start_time = Instant::now();
    color_eyre::install().expect("Failed to install color_eyre");
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or("info,serenity=error,tracing=error"),
    );
    let db_connection = establish_connection().await?;
    info!("Logining into Discord...");
    let _guild_id = GuildId(
        env::var("DEBUG_GUILD")
            .expect("Expected DEBUG_GUILD in environment")
            .parse()
            .expect("DEBUG_GUILD must be an integer"),
    );
    let commands = vec![
        achivit_rs::wiki::wiki(),
        achivit_rs::time::ping(),
        achivit_rs::time::uptime(),
        achivit_rs::time::server_time(),
        achivit_rs::time::random_event(),
        achivit_rs::dev_tools::list_slash_commands(),
        achivit_rs::manage_users::register_character(),
        achivit_rs::manage_users::delete_character(),
        achivit_rs::lookup_df::lookup_df_character(),
        achivit_rs::lookup_df::compare_df_characters(),
        achivit_rs::lookup_df::roles_list(),
        achivit_rs::guild_settings::set_roles(),
    ];
    // let test_commands = vec![achivit_rs::lookup_df::lookup_df_roles()];
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
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("|".into()),
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
                // poise::builtins::register_in_guild(ctx, &test_commands, _guild_id).await?;
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
