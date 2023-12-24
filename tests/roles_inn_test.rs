extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::{requirements::{check_requirements,get_requirements}, parsing::{FileFetcher,ParsingCategory}};
    use color_eyre::Result;
    #[test]
    fn inn_loads() -> Result<()> {
        get_requirements("InnList.json")?;
        Ok(())
    }
    #[tokio::test]
    async fn roles_amount()->Result<()>{
        let ruleofnine = FileFetcher::new("htmls/ruleofnine.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_character_data()?;
        let ascends = check_requirements(&ruleofnine, "ascendancies.json")?;
        assert_eq!(11,ascends.requirements().len());
        let roles = check_requirements(&ruleofnine, "roles.json")?;
        assert_eq!(14,roles.requirements().len());
        let just_name = FileFetcher::new("htmls/just_name.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_character_data()?;
        let ascends = check_requirements(&just_name, "ascendancies.json")?;
        let roles = check_requirements(&just_name, "roles.json")?;
        assert_eq!(0,ascends.requirements().len());
        assert_eq!(0,roles.requirements().len());
        let ach = FileFetcher::new("htmls/3ach.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_character_data()?;
        let ascends = check_requirements(&ach, "ascendancies.json")?;
        let roles = check_requirements(&ach, "roles.json")?;
        assert_eq!(5,ascends.requirements().len());
        assert_eq!(17,roles.requirements().len());
        Ok(())
    }
}
