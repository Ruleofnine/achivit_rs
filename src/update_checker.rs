use crate::{
    requests::{fetch_page_with_user_agent, DESIGN_NOTES_LINK, USER_AGENT},
    Context, Error, Task,
    embeds::send_update_embed
};
use color_eyre::{eyre::eyre, Result};
use getset::Getters;
use scraper::{Html, Selector};
use tokio::time::{self, Duration};
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
async fn run_update_checker(ctx: Context<'_>) -> Result<()> {
    let tasks = ctx.data().tasks().clone();
    let dn_str = fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK).await?;
    let dn = DesignNote::parse_from_str(&dn_str)?;
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        while tasks.is_running(UPDATE_CHECKER).await {
            interval.tick().await;
            let dn_str = fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK)
                .await
                .unwrap();
            let new_dn = DesignNote::parse_from_str(&dn_str).unwrap();
            if new_dn.date() != dn.date() {
                tasks.stop_task(UPDATE_CHECKER).await;
                send_update_embed(new_dn).await;
                // send message to channel with ctx.send()
            }
        }
    });
    Ok(())
}
/// Check Design Notes for update every 10 seconds
#[poise::command(prefix_command,)]
pub async fn update_checker(ctx: Context<'_>) -> Result<(), Error> {
    let tasks = ctx.data().tasks();
    let is_running = tasks.is_running(UPDATE_CHECKER).await;
    match is_running {
        true => {
            ctx.reply("Update checker already running").await?;
        }
        false => {
            tasks.start_task(UPDATE_CHECKER).await;
            run_update_checker(ctx).await?
        }
    }
    Ok(())
}
