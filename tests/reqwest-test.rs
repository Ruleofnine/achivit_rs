extern crate achivit_rs;
#[cfg(any(feature = "reqwest-tests",rust_analyzer))]
mod tests {
    use achivit_rs::lookup_df::LookupCategory;
    use achivit_rs::parsing::CharacterFetcher;
    use color_eyre::Result;
    #[tokio::test]
    async fn character_lookup() -> Result<()> {
        let char = CharacterFetcher::new(4211037, LookupCategory::CharacterPage)
            .fetch_data()
            .await?.to_lookupstate()?.extract_character_data()?;
        assert_eq!(*char.id(),4211037);
        assert_eq!(char.name(),"Ruleofnine");
        assert_eq!(*char.level(),90);
        assert_eq!(char.dmk().as_ref(),Some(&"Master Doom Knight".to_string()));
        Ok(())
    }
    #[tokio::test]
    async fn character_lookup_roles() -> Result<()> {
        let char = CharacterFetcher::new(4211037, LookupCategory::CharacterPage)
            .category(achivit_rs::parsing::ParsingCategory::Items)
            .fetch_data()
            .await?.to_lookupstate()?.extract_character_data()?;
        assert_eq!(*char.id(),4211037);
        assert_eq!(char.name(),"Ruleofnine");
        assert_eq!(*char.level(),90);
        assert_eq!(char.dmk().as_ref(),Some(&"Master Doom Knight".to_string()));
        Ok(())
    }
}
