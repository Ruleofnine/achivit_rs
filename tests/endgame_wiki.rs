extern crate achivit_rs;
#[cfg(any(feature = "reqwest-tests",rust_analyzer))]
mod tests {
    use achivit_rs::requests::{fetch_page_with_user_agent, USER_AGENT};
    use color_eyre::Result;
    use serde_derive::{Deserialize, Serialize};
    use achivit_rs::wiki::{Page,SeedPage};

    #[tokio::test]
    async fn wiki_test() -> Result<()> {
        let seed =  "https://dragonfable-endgame.fandom.com/wikia.php?controller=SearchSeeding&method=getLocalSearchInfo&format=json";
        let query = "arch";
        let url_part_1 = "https://dragonfable-endgame.fandom.com/wikia.php?controller=UnifiedSearchSuggestions&method=getSuggestions&query=";
        let url_part_2 = "&format=json&scope=internal";
        let url = format!("{}{}{}", url_part_1, query, url_part_2);
        let page = fetch_page_with_user_agent(USER_AGENT, &url).await?;
        let page_data: Page = serde_json::from_str(&page)?;
        Ok(())
    }
    #[tokio::test]
    async fn blank_query_test() -> Result<()> {
        let seed =  "https://dragonfable-endgame.fandom.com/wikia.php?controller=SearchSeeding&method=getLocalSearchInfo&format=json";
        let page = fetch_page_with_user_agent(USER_AGENT, &seed).await?;
        let page_data: SeedPage = serde_json::from_str(&page)?;
        Ok(())
    
}
}
