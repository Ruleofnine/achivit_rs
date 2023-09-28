use color_eyre::Result;
use dfelp::requests::fetch_page;
use crate::{Error, Context};
pub async fn get_wiki(search_query: &str) -> Result<String,> {
    let search_query = if search_query.contains(' ') { search_query.split_whitespace()
        .map(|word| {
            match word.to_lowercase().as_str() {
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
            }
        })
        .collect::<Vec<String>>()
        .join("_")} else { search_query.to_string()};
    let url = dbg!(format!(
        "https://dragonfable-endgame.fandom.com/wiki/{}",
        search_query
    ));
    match fetch_page(&url).await{
        Ok(_) => {Ok(url)}
        Err(_) => {Err(color_eyre::eyre::eyre!(url))}
    }
}
/// Query the dragonfable Endgame Wiki
#[poise::command(slash_command)]
pub async fn wiki(
    ctx: Context<'_>, 
    #[description = "Dragonfable Endgame Wiki Qurey"] search_query: String
) -> Result<(), Error> {
     match get_wiki(&search_query).await {
        Ok(url) =>  ctx.say(url).await?,
        Err(e) => ctx.say(format!("Page not found! {}",e)).await?,
    };
    Ok(())
}
