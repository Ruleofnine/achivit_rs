extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::lookup_df::LookupState;
    use achivit_rs::requirements::get_requirements;
    use achivit_rs::paginate::get_requirement_pages;
    use color_eyre::Result;
    use achivit_rs::parsing::{ParsingCategory,FileFetcher};


    #[test]
    fn paginate_test() -> Result<()> {
        let reqs = get_requirements("InnList.json")?;
        let pages = get_requirement_pages(reqs,None);
        pages.iter().for_each(|page|assert!(page.len()<4096));
        Ok(())
    }
    #[tokio::test]
    async fn paginate_inventory_test() -> Result<()> {
        let (_,pages) = FileFetcher::new("htmls/3ach.html")
            .category(ParsingCategory::Inventory)
            .fetch_data()
            .await?
            .to_lookupstate()?.extract_inventory_data()?;
        pages.iter().for_each(|page|assert!(dbg!(page.len())<4096));
        Ok(())
    }
}
