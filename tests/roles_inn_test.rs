extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::{roles::{get_inn_list, check_roles}, parsing::{FileFetcher,ParsingCategory}};
    use color_eyre::Result;
    #[test]
    fn inn_loads() -> Result<()> {
        get_inn_list()?;
        Ok(())
    }
    #[tokio::test]
    async fn roles_amount()->Result<()>{
        let ruleofnine = FileFetcher::new("htmls/ruleofnine.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_data()?;
        let ascends = check_roles(&ruleofnine, "ascendancies.json")?;
        assert_eq!(11,ascends.roles().len());
        let roles = check_roles(&ruleofnine, "roles.json")?;
        assert_eq!(14,roles.roles().len());
        let just_name = FileFetcher::new("htmls/just_name.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_data()?;
        let ascends = check_roles(&just_name, "ascendancies.json")?;
        let roles = check_roles(&just_name, "roles.json")?;
        assert_eq!(0,ascends.roles().len());
        assert_eq!(0,roles.roles().len());
        Ok(())
    }
}
