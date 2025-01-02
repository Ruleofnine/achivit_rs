extern crate achivit_rs;
#[cfg(any(feature = "reqwest-tests", rust_analyzer))]
mod tests {
    use achivit_rs::lookup_df::LookupCategory;
    use achivit_rs::parsing::{parse_aqc_charpage, parse_aqw_charpage, parse_mech_quest_charpage, CharacterFetcher};
    use achivit_rs::requests::{
        fetch_json, fetch_page_with_user_agent, get_random_event, FLASH_USER_AGENT, USER_AGENT,
    };
    use color_eyre::Result;
    use scraper::{Html, Selector};
    #[tokio::test]
    async fn character_lookup() -> Result<()> {
        let char = CharacterFetcher::new(4211037, LookupCategory::CharacterPage)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?;
        assert_eq!(*char.id(), 4211037);
        assert_eq!(char.name(), "Ruleofnine");
        assert_eq!(*char.level(), 90);
        assert_eq!(char.dmk().as_ref(), Some(&"Master Doom Knight".to_string()));
        Ok(())
    }
    #[tokio::test]
    async fn character_lookup_roles() -> Result<()> {
        let char = CharacterFetcher::new(4211037, LookupCategory::CharacterPage)
            .category(achivit_rs::parsing::ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?;
        assert_eq!(*char.id(), 4211037);
        assert_eq!(char.name(), "Ruleofnine");
        assert_eq!(*char.level(), 90);
        assert_eq!(char.dmk().as_ref(), Some(&"Master Doom Knight".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn mechquest_lookup() -> Result<()> {
        let url = format!("https://account.mechquest.com/CharPage?id={}", 2);
        let json_string = fetch_page_with_user_agent(USER_AGENT, &url).await?;
        let document = Html::parse_document(&json_string);
        let mechquestdata = parse_mech_quest_charpage(document);
        Ok(())
    }

    #[tokio::test]
    async fn aqc_lookup() -> Result<()> {
        let url = format!("https://aq.battleon.com/game/flash/charview?temp={}", 22);
        let json_string = fetch_page_with_user_agent(FLASH_USER_AGENT, &url).await?;
        let document = Html::parse_document(&json_string);
        let data = parse_aqc_charpage(document)?;
        dbg!(data);
        Ok(())
    }
    #[tokio::test]
    async fn random_event_test() -> Result<()> {
        let data = get_random_event().await;
        dbg!(data);
        Ok(())
    }
    #[tokio::test]
    async fn aqw_lookup() -> Result<()> {
        let username = "Artix";
        let url = format!("https://account.aq.com/CharPage?id={username}");
        let json_string = fetch_page_with_user_agent(FLASH_USER_AGENT, &url).await?;
        let document = Html::parse_document(&json_string);
        let data = parse_aqw_charpage(document).await?;
        dbg!(data);
        Ok(())
    }
}
