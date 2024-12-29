use crate::requests::{fetch_page_with_user_agent, USER_AGENT};
use crate::serenity::Color;
use crate::{Context, Error};
use color_eyre::Result;
use poise::AutocompleteChoice;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
const SEED_URL:&str =  "https://dragonfable-endgame.fandom.com/wikia.php?controller=SearchSeeding&method=getLocalSearchInfo&format=json";
const SEARCH_URL_PART_1:&str = "https://dragonfable-endgame.fandom.com/wikia.php?controller=UnifiedSearchSuggestions&method=getSuggestions&query=";
const SEARCH_URL_PART_2: &str = "&format=json&scope=internal";
pub async fn get_wiki(search_query: &str) -> Result<String> {
    let search_query = if search_query.contains(' ') {
        search_query
            .split_whitespace()
            .map(|word| match word.to_lowercase().as_str() {
                "of" | "the" | "that" | "and" => word.to_string(),
                _ => {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first_char) => {
                            first_char.to_uppercase().collect::<String>() + chars.as_str()
                        }
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("_")
    } else {
        search_query.to_string()
    };
    let url = format!(
        "https://dragonfable-endgame.fandom.com/wiki/{}",
        search_query
    );
    match fetch_page_with_user_agent(USER_AGENT, &url).await {
        Ok(_) => Ok(url),
        Err(_) => Err(color_eyre::eyre::eyre!(url)),
    }
}
/// Query the dragonfable Endgame Wiki
#[poise::command(slash_command)]
pub async fn wiki(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_wiki"]
    #[description = "Dragonfable Endgame Wiki Qurey"]
    search_query: String,
) -> Result<(), Error> {
    match get_wiki(&search_query).await {
        Ok(url) =>  ctx.say(url).await?,
        Err(e) => {ctx.send( |f| {
            f.embed(|f| f.title(format!("[{}] Wiki Page Not Found",search_query)).color(Color::DARK_RED).description(e).thumbnail("https://static.wikia.nocookie.net/dragonfable-endgame/images/e/e6/Site-logo.png/revision/latest?cb=20210713144829"))
        }).await?}
    };
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedPage {
    search_phrases: Vec<SearchPhrase>,
    featured: Featured,
    sponsored: Sponsored,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchPhrase {
    term: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sponsored {
    title: String,
    url: String,
    pixel: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Featured {
    title: String,
    url: String,
    image: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    query: String,
    ids: Value,
    suggestions: Vec<String>,
}

pub async fn autocomplete_wiki(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<AutocompleteChoice<String>> {
    if partial.is_empty() {
        let page = fetch_page_with_user_agent(USER_AGENT, SEED_URL)
            .await
            .expect("failed to seed wiki url");
        let page_data: SeedPage = serde_json::from_str(&page).unwrap();
        return page_data
            .search_phrases
            .iter()
            .map(|f| AutocompleteChoice {
                name: f.term.to_string(),
                value: f.term.to_string(),
            })
            .collect::<Vec<poise::AutocompleteChoice<String>>>();
    } else {
        let url = format!("{}{}{}", SEARCH_URL_PART_1, partial, SEARCH_URL_PART_2);
        let page = fetch_page_with_user_agent(USER_AGENT, &url)
            .await
            .expect("failed to seed wiki url");
        let page_data: Page = serde_json::from_str(&page)
            .map_err(|e| dbg!((page, e)))
            .unwrap();
        if page_data.ids.is_array() {
            return Vec::new();
        } else {
            let ids: HashMap<String, u32> = serde_json::from_value(page_data.ids)
                .expect("failed to turn Value to HashMap of ids");
            ids.iter()
                .map(|(name, _)| poise::AutocompleteChoice {
                    name: name.to_string(),
                    value: name.to_string(),
                })
                .collect()
        }
    }
}
