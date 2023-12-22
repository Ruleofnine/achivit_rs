pub mod db;
pub mod requirements;
pub mod dev_tools;
pub mod roles_extended;
pub mod embeds;
pub mod error_handler;
pub mod event_handler;
pub mod lookup_df;
pub mod manage_users;
pub mod parsing;
pub mod requests;
pub mod rng;
pub mod sheets;
pub mod time;
pub mod wiki;
pub mod guild_settings;
pub use crate::event_handler::event_handler;
pub use dotenv::dotenv;
pub use log::info;
pub use lookup_df::lookup_df_character;
pub use poise::serenity_prelude as serenity;
pub mod paginate;
use sqlx::PgPool;
use std::time::Instant;
use std::env;
use std::path::Path;
use color_eyre::owo_colors::{OwoColorize,Style};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    pub start_time: Instant,
    pub db_connection: PgPool,
}
pub fn get_command_list()-> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>>{
   vec![
        crate::wiki::wiki(),
        crate::time::ping(),
        crate::time::uptime(),
        crate::time::server_time(),
        crate::time::random_event(),
        crate::dev_tools::list_slash_commands(),
        crate::manage_users::register_character(),
        crate::manage_users::delete_character(),
        crate::lookup_df::lookup_df_character(),
        crate::lookup_df::compare_df_characters(),
        crate::lookup_df::roles_list(),
        crate::guild_settings::set_roles(),
        crate::roles_extended::inn_items(),
    ]
}
pub fn print_banner(){
    let achivit_style = Style::new().bright_purple();
    let prefix_style = Style::new().bright_cyan();
    let postfix_style = Style::new().bright_yellow();
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let repo = env!("CARGO_PKG_REPOSITORY");
    let exec_path =match env::current_exe() {
        Ok(exe_path) => {
            exe_path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap_or("No Path").to_owned()
        }
        Err(_) => {
            "path error".to_owned()
        }
    };

println!("{}",  "      .o.                 oooo         o8o               o8o      .".style(achivit_style));
println!("{}     {}{}",r#"     .888.                `888         `"'               `"'    .o8"#.style(achivit_style),"Path:".style(prefix_style),exec_path.style(postfix_style));
println!("{}   {}{}",r#"    .8"888.      .ooooo.   888 .oo.   oooo  oooo    ooo oooo  .o888oo"#.style(achivit_style),"Author:".style(prefix_style),authors.style(postfix_style));
println!("{}     {}{}",r#"   .8' `888.    d88' `"Y8  888P"Y88b  `888   `88.  .8'  `888    888"#.style(achivit_style),"Version:".style(prefix_style),version.style(postfix_style));
println!("{}     {}{} ",r#"  .88ooo8888.   888        888   888   888    `88..8'    888    888"#.style(achivit_style),"Description:".style(prefix_style),description.style(postfix_style));
println!("{}   {}{}",r#" .8'     `888.  888   .o8  888   888   888     `888'     888    888 ."#.style(achivit_style),"Repository:".style(prefix_style),repo.style(postfix_style));
println!("{}    {}{}",r#"o88o     o8888o `Y8bod8P' o888o o888o o888o     `8'     o888o   "888"#.style(achivit_style),"Discord:".style(prefix_style),"https://discord.gg/UrKUVDVCrv".style(postfix_style));
}
#[macro_export]
macro_rules! create_getters {
    ($($name:ident : $type:ty),*) => {
        $(
            pub fn $name(&self) -> &$type {
                &self.$name
            }
        )*
    };
}
