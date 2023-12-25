use crate::requests::{fetch_json, fetch_page_with_user_agent, DESIGN_NOTES_LINK, USER_AGENT};
use chrono::NaiveDate;
use color_eyre::{eyre::eyre, Result};
use scraper::{Html, Selector};
use getset::Getters;
#[derive(Debug,Getters)]
#[get="pub"]
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
        let date_str = article
            .select(&date_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        let date = NaiveDate::parse_from_str(&date_str, "%A, %B %d, %Y")?;
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
