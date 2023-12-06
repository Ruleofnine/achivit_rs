pub mod db;
pub mod dev_tools;
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
pub use crate::event_handler::event_handler;
pub use dotenv::dotenv;
pub use log::info;
pub use lookup_df::lookup_df_character;
pub use poise::serenity_prelude as serenity;
use sqlx::PgPool;
use std::time::Instant;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    pub start_time: Instant,
    pub db_connection: PgPool,
}
