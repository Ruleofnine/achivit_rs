use crate::parsing::ElementRefWrapper;
use crate::{
    embeds::send_update_embed,
    guild_settings::GuildSettings,
    requests::{fetch_page_with_user_agent, DESIGN_NOTES_LINK, USER_AGENT},
    Context, Error, Task,
};
use chrono::{Datelike, NaiveDate};
use color_eyre::{eyre::eyre, Result};
use getset::Getters;
use log::{error, info};
use scraper::{Html, Selector};
use sqlx::{query_as, FromRow};
use std::sync::Arc;
use tokio::time::{self, Duration};
static DATE_FORMATS: [&str; 27] = [
    "%A, %B %d, %Y",
    "%A, %B %-d, %Y",
    "%A, %b %d, %Y",
    "%A, %b %-d, %Y",
    "%B %d, %Y",
    "%B %-d, %Y",
    "%b %d, %Y",
    "%b %-d, %Y",
    "%Y-%m-%d",
    "%d-%m-%Y",
    "%m-%d-%Y",
    "%Y/%m/%d",
    "%d/%m/%Y",
    "%m/%d/%Y",
    "%m/%-d/%Y",
    "%B %-d, %Y",
    "%A, Dec. %d, %Y",
    "%A, Dec %d, %Y",
    "%A, Dez %d, %Y",
    "%A %B %d %Y",
    "%A, %d %B %Y",
    "%a, %B %d, %Y",
    "%a, %b %d, %Y",
    "%B %dst, %Y",
    "%B %dnd, %Y",
    "%B %drd, %Y",
    "%B %dth, %Y",
];
#[derive(poise::ChoiceParameter, PartialEq, Debug,Clone, Copy)]
pub enum UpdateCheckerFeatureFlag {
    Start,
    Stop,
    Force,
    ForceNoPing,
    CheckIsRunning,
}
const UPDATE_CHECKER: &str = "update_checker";
#[derive(Debug, Getters)]
#[get = "pub"]
pub struct DesignNote {
    update_name: String,
    link: String,
    date: NaiveDate,
    image: String,
    poster_name: String,
    poster_image: String,
}
impl DesignNote {
    fn new(
        update_name: String,
        date: NaiveDate,
        link: String,
        image: String,
        poster_name: String,
        poster_image: String,
    ) -> DesignNote {
        DesignNote {
            update_name,
            date,
            link,
            image,
            poster_name,
            poster_image,
        }
    }

    pub fn parse_from_str(dn_str: &str) -> Result<DesignNote> {
        let document = Html::parse_document(dn_str);
        let article_selector = Selector::parse("div.col.pt-2.dn-article").unwrap();
        let article = document
            .select(&article_selector)
            .next()
            .ok_or(eyre!("Unable to find article"))?;
        let date_selector = Selector::parse("p.mb-0").unwrap();
        let date_text =
            ElementRefWrapper(article.select(&date_selector).next().unwrap()).to_string();

        let parsed_date = match NaiveDate::parse_from_str(&date_text, "%A, %B %d, %Y") {
            Ok(date) => date,
            Err(e) => {
                error!("Error parsing date in DN: String = [{date_text}] Error = [{e}]");

                if let Some(date) = DATE_FORMATS.iter().find_map(|format| {
                    match NaiveDate::parse_from_str(&date_text, format) {
                        Ok(date) => {
                            info!("Successfully parsed date with format: {}", format);
                            Some(date)
                        }
                        Err(_) => None,
                    }
                }) {
                    date // Successfully parsed date
                } else {
                    error!("Could not parse date: {}", date_text);
                    let today = chrono::Local::now().date_naive();
                    error!("Falling back to today's date: {}", today);
                    today
                }
            }
        };
        let link_selector = Selector::parse("h2.postTitle.pt-0 a").unwrap();
        let link = article
            .select(&link_selector)
            .next()
            .ok_or(eyre!("Unable to find link"))?
            .value()
            .attr("href")
            .ok_or(eyre!("Href attribute not found"))?;

        let update_name_selector = Selector::parse("h2.postTitle.pt-0").unwrap();
        let update_name_element = article
            .select(&update_name_selector)
            .next()
            .ok_or(eyre!("Unable to find update name"))?;

        let update_name = update_name_element
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let image_selector = Selector::parse("div.col.pt-2.dn-article img").unwrap();
        let image = article
            .select(&image_selector)
            .next()
            .ok_or(eyre!("Unable to find image"))?
            .value()
            .attr("src")
            .ok_or(eyre!("Src attribute not found"))?;

        let poster_image_selector = Selector::parse("a.d-block img").unwrap();
        let poster_image = document
            .select(&poster_image_selector)
            .next()
            .ok_or(eyre!("Unable to find poster image"))?
            .value()
            .attr("src")
            .ok_or(eyre!("Src attribute not found"))?;
        let poster_image = format!("https://www.dragonfable.com{}", poster_image);

        let poster_name_selector =
            Selector::parse("div.d-none.d-md-block.dnAvatar.col-auto.text-center.pt-2 p").unwrap();
        let poster_name = document
            .select(&poster_name_selector)
            .next()
            .ok_or(eyre!("Unable to find poster name"))?
            .inner_html();
        Ok(DesignNote::new(
            update_name,
            parsed_date,
            link.to_string(),
            image.to_string(),
            poster_name,
            poster_image,
        ))
    }
}
async fn run_update_checker(
    ctx: Context<'_>,
    flag: UpdateCheckerFeatureFlag,
    all_guilds: Arc<Vec<GuildSettings>>,
) -> Result<()> {
    let tasks = ctx.data().tasks().clone_inner();
    let now = chrono::Local::now();
    let dn_url = format!("{DESIGN_NOTES_LINK}/{}/{}",now.year(),now.month());
    let last_dn_str = fetch_page_with_user_agent(USER_AGENT, &dn_url).await?;
    let last_dn = DesignNote::parse_from_str(&last_dn_str)?;
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        while tasks.is_running(UPDATE_CHECKER).await {
            interval.tick().await;
            let new_dn_str = match fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK).await {
                Ok(data) => data,
                Err(e) => {
                    error!("Failed to fetch page for DN with error: [{e}]");
                    continue;
                }
            };
            let new_dn = match DesignNote::parse_from_str(&new_dn_str) {
                Ok(note) => note,
                Err(e) => {
                    error!("failed to parse DN with error: [{e}]");
                    continue;
                }
            };
            if new_dn.date() > last_dn.date() || flag == UpdateCheckerFeatureFlag::Force || flag == UpdateCheckerFeatureFlag::ForceNoPing {
                tasks.stop_task(UPDATE_CHECKER).await;
                if let Err(e) = send_update_embed(Arc::clone(&all_guilds), new_dn,flag).await {
                    error!("Failed to send Embed for DN with error: [{e}]")
                }
                // send message to channel with ctx.send()
            }
        }
    });
    Ok(())
}
#[derive(FromRow, Debug)]
struct AnnouncementSettings {
    announcement_channel_id: Option<i64>,
    announcement_role_id: Option<i64>,
}
impl AnnouncementSettings {
    fn is_set(&self) -> bool {
        self.announcement_channel_id.is_some() && self.announcement_role_id.is_some()
    }
}
/// Check Design Notes for update every 10 seconds
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR",
    guild_only
)]
pub async fn update_checker(ctx: Context<'_>, flag: UpdateCheckerFeatureFlag) -> Result<(), Error> {
    let tasks = ctx.data().tasks();
    let pool = ctx.data().db();
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let is_running = tasks.is_running(UPDATE_CHECKER).await;
    let prefix = ctx
        .framework()
        .options()
        .prefix_options
        .prefix
        .as_deref()
        .unwrap_or_default();
    let guild_settings = query_as!(AnnouncementSettings,"select announcement_channel_id,announcement_role_id from guild_settings where guild_id = $1",guild_id)
        .fetch_optional(pool).await?;
    match guild_settings {
        Some(settings) => {
            if !settings.is_set() {
                {
                    ctx.reply(format!("No Announcement Settings for this guild\nUse \"{prefix}init_announcements #channel @role\" to set up Announcements for this guild")).await?;
                    return Ok(());
                }
            }
            settings
        }
        None => {
            ctx.reply(
                format!("No Guild registered with this Guild ID.\n\"{prefix}init_guild\" to register Guild."),
            )
            .await?;
            return Ok(());
        }
    };
    let all_guilds = query_as!(GuildSettings,"select * from guild_settings where announcement_channel_id IS NOT NULL and announcement_role_id IS NOT NULL").fetch_all(pool).await?;
    let arc_guilds = Arc::new(all_guilds);
    match (&flag, is_running) {
        (UpdateCheckerFeatureFlag::Start, false) | (UpdateCheckerFeatureFlag::Force, _) | (UpdateCheckerFeatureFlag::ForceNoPing, _) => {
            tasks.start_task(UPDATE_CHECKER).await;
            run_update_checker(ctx, flag, arc_guilds).await?;
            ctx.reply(format!("Command Executed Successfully with Feature Flag {}",flag)).await?;
        }
        (UpdateCheckerFeatureFlag::Start, true) => {
            ctx.reply("Update Checker is **already** running!").await?;
        }
        (UpdateCheckerFeatureFlag::Stop, true) => {
            tasks.stop_task(UPDATE_CHECKER).await;
            ctx.reply("Update Checker is **STOPPED!**").await?;
        }
        (UpdateCheckerFeatureFlag::Stop, false) => {
            ctx.reply("Update Checker is **not** running!").await?;
        }
        (UpdateCheckerFeatureFlag::CheckIsRunning, _) => {
            ctx.reply(format!("Update checker Status: {is_running}"))
                .await?;
        }
    }
    Ok(())
}
