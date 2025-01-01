pub mod db;
pub mod dev_tools;
pub mod embeds;
pub mod error_handler;
pub mod event_handler;
pub mod guild_settings;
pub mod lookup_df;
pub mod manage_users;
pub mod parsing;
pub mod requests;
pub mod requirements;
pub mod rng;
pub mod roles_extended;
pub mod sheets;
pub mod time;
pub mod wiki;
pub mod mech_aqw_lookup;
pub use crate::event_handler::event_handler;
pub use dotenv::dotenv;
pub use log::info;
pub use lookup_df::lookup_df_character;
pub use poise::serenity_prelude as serenity;
pub mod paginate;
pub mod update_checker;
use crate::serenity::Mutex;
use color_eyre::owo_colors::{OwoColorize, Style};
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub fn str_to_i64(s: &str) -> i64 {
    s.chars().map(|c| c as i64).sum()
}

pub struct Tasks {
    inner: Arc<Mutex<HashMap<String, bool>>>,
}

trait Task{
    async fn stop_task(&self,task_name:&str);
    async fn is_running(&self,task_name:&str)->bool;
}
impl Task for Arc<Mutex<HashMap<String, bool>>>{
    async fn stop_task(&self, task_name: &str) {
        let mut tasks = self.lock().await;
        tasks.insert(task_name.to_string(), false);
    }

    async fn is_running(&self, task_name: &str) -> bool {
        let tasks = self.lock().await;
        *tasks.get(task_name).unwrap_or(&false)
    }

} 
impl Tasks {
    pub fn default() -> Self {
        Tasks {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn clone(&self)->Arc<Mutex<HashMap<String, bool>>>{
        self.inner.clone()

    }
    pub async fn start_task(&self, task_name: &str) {
        let mut tasks = self.inner.lock().await;
        tasks.insert(task_name.to_string(), true);
    }

    pub async fn stop_task(&self, task_name: &str) {
        let mut tasks = self.inner.lock().await;
        tasks.insert(task_name.to_string(), false);
    }

    pub async fn is_running(&self, task_name: &str) -> bool {
        let tasks = self.inner.lock().await;
        *tasks.get(task_name).unwrap_or(&false)
    }
}

pub struct Data {
    pub start_time: Instant,
    pub db_connection: PgPool,
    pub tasks: Tasks,
    pub super_users: Vec<u64>
}
impl Data {
    pub fn tasks(&self) -> &Tasks {
        &self.tasks
    }
    pub fn new(start_time:Instant,db_connection:PgPool,super_users:Vec<u64>)->Data{
        Data { start_time, db_connection, tasks: Tasks::default(),super_users }
    }
    pub fn db(&self)->&PgPool{
        &self.db_connection
    }
}
pub fn get_command_list(
) -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        crate::wiki::wiki(),
        crate::time::ping(),
        crate::time::uptime(),
        crate::time::server_time(),
        crate::time::random_event(),
        crate::dev_tools::list_slash_commands(),
        crate::dev_tools::clear_guild_slash_commands(),
        crate::manage_users::register_character(),
        crate::manage_users::delete_character(),
        crate::lookup_df::lookup_df_character(),
        crate::lookup_df::compare_df_characters(),
        crate::lookup_df::roles_list(),
        crate::mech_aqw_lookup::lookup_mechquest_id(),
        crate::mech_aqw_lookup::lookup_aqc_id(),
        crate::mech_aqw_lookup::lookup_aqw_character(),
        crate::guild_settings::set_roles(),
        crate::guild_settings::leave_guild(),
        crate::guild_settings::set_ascends(),
        crate::guild_settings::init_guild(),
        crate::guild_settings::init_announcements(),
        crate::guild_settings::set_inn_items(),
        crate::roles_extended::inn_items(),
        crate::update_checker::update_checker(),
    ]
}
pub fn print_banner() {
    let achivit_style = Style::new().bright_purple();
    let prefix_style = Style::new().bright_cyan();
    let postfix_style = Style::new().bright_yellow();
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let repo = env!("CARGO_PKG_REPOSITORY");
    let exec_path = match env::current_exe() {
        Ok(exe_path) => exe_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap_or("No Path")
            .to_owned(),
        Err(_) => "path error".to_owned(),
    };

    println!(
        "{}",
        "      .o.                 oooo         o8o               o8o      .".style(achivit_style)
    );
    println!(
        "{}     {}{}",
        r#"     .888.                `888         `"'               `"'    .o8"#
            .style(achivit_style),
        "Path:".style(prefix_style),
        exec_path.style(postfix_style)
    );
    println!(
        "{}   {}{}",
        r#"    .8"888.      .ooooo.   888 .oo.   oooo  oooo    ooo oooo  .o888oo"#
            .style(achivit_style),
        "Author:".style(prefix_style),
        authors.style(postfix_style)
    );
    println!(
        "{}     {}{}",
        r#"   .8' `888.    d88' `"Y8  888P"Y88b  `888   `88.  .8'  `888    888"#
            .style(achivit_style),
        "Version:".style(prefix_style),
        version.style(postfix_style)
    );
    println!(
        "{}     {}{} ",
        r#"  .88ooo8888.   888        888   888   888    `88..8'    888    888"#
            .style(achivit_style),
        "Description:".style(prefix_style),
        description.style(postfix_style)
    );
    println!(
        "{}   {}{}",
        r#" .8'     `888.  888   .o8  888   888   888     `888'     888    888 ."#
            .style(achivit_style),
        "Repository:".style(prefix_style),
        repo.style(postfix_style)
    );
    println!(
        "{}    {}{}",
        r#"o88o     o8888o `Y8bod8P' o888o o888o o888o     `8'     o888o   "888"#
            .style(achivit_style),
        "Discord:".style(prefix_style),
        "https://discord.gg/UrKUVDVCrv".style(postfix_style)
    );
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
