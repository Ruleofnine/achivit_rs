use std::sync::Arc;
use crate::{
    embeds::send_update_embed,
    requests::{fetch_page_with_user_agent, DESIGN_NOTES_LINK, USER_AGENT},
    Context, Error, Task, guild_settings::GuildSettings,
};
use color_eyre::{eyre::eyre, Result};
use getset::Getters;
use scraper::{Html, Selector};
use sqlx::{query_as,FromRow};
use tokio::time::{self, Duration};

#[derive(poise::ChoiceParameter, PartialEq, Debug)]
enum UpdateCheckerFeatureFlag {
    Start,
    Stop,
    Force,
    CheckIsRunning,
}
const UPDATE_CHECKER: &str = "update_checker";
#[derive(Debug, Getters)]
#[get = "pub"]
pub struct DesignNote {
    update_name: String,
    link: String,
    date: String,
    image: String,
    poster_name: String,
    poster_image: String,
}
impl DesignNote {
    fn new(
        update_name: String,
        date: String,
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
        let date = article
            .select(&date_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

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
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

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
            date,
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
    all_guilds:Arc<Vec<GuildSettings>>
) -> Result<()> {
    let tasks = ctx.data().tasks().clone();
    let last_dn_str = fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK).await?;
    let last_dn = DesignNote::parse_from_str(&last_dn_str)?;
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        while tasks.is_running(UPDATE_CHECKER).await {
            interval.tick().await;
            let new_dn_str = fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK)
                .await
                .unwrap();
            let new_dn = DesignNote::parse_from_str(&new_dn_str).unwrap();
            if new_dn.date() != last_dn.date() || flag == UpdateCheckerFeatureFlag::Force {
                tasks.stop_task(UPDATE_CHECKER).await;
                send_update_embed(Arc::clone(&all_guilds), new_dn)
                    .await
                    .expect("Failed to send update embed");
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
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", default_member_permissions = "ADMINISTRATOR",guild_only)]
pub async fn update_checker(ctx: Context<'_>, flag: UpdateCheckerFeatureFlag) -> Result<(), Error> {
    let tasks = ctx.data().tasks();
    let pool = ctx.data().db();
    let guild_id = ctx.guild_id().unwrap().0 as i64;
    let is_running = tasks.is_running(UPDATE_CHECKER).await;
    let guild_settings = query_as!(AnnouncementSettings,"select announcement_channel_id,announcement_role_id from guild_settings where guild_id = $1",guild_id)
        .fetch_optional(pool).await?;
    match guild_settings {
        Some(settings) => {
            if !settings.is_set() {
                {
                    ctx.reply("No Announcement Settings for this guild\nUse \"|init_announcements #channel @role\" to set up Announcements for this guild").await?;
                    return Ok(())
                }
            }
            settings
        }
        None => {
            ctx.reply(
                "No Guild registered with this Guild ID.\n\"|init_guild\" to register Guild.",
            )
            .await?;
            return Ok(())
        }
    };
    let all_guilds = query_as!(GuildSettings,"select * from guild_settings where announcement_channel_id IS NOT NULL and announcement_role_id IS NOT NULL").fetch_all(pool).await?;
    let arc_guilds = Arc::new(all_guilds);
    match (&flag, is_running) {
        (UpdateCheckerFeatureFlag::Start, false) | (UpdateCheckerFeatureFlag::Force, _) => {
            tasks.start_task(UPDATE_CHECKER).await;
            run_update_checker(ctx,flag,arc_guilds).await?;
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
