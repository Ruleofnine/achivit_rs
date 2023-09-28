use crate::requests::fetch_page;
use color_eyre::Result;
pub async fn get_wiki(search_query: &str) -> Result<()> {
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
    let _ = fetch_page(&url).await?;
    Ok(())
}
