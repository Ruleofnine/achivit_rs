extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::parsing::*;
    use achivit_rs::lookup_df::LookupState;
    use color_eyre::Result;
    #[test]
    fn parse_pages() -> Result<()>{
        let test_page = "htmls/ruleofnine.html";
        let a = parse_df_character_from_file(&test_page)?;
        if let LookupState::CharacterPage(data) = a {
            assert_eq!(data.name,"Ruleofnine".to_owned());
            assert_eq!(data.level,90);
            assert_eq!(data.wars.calc_waves_cleared(),10009);
            assert_eq!(data.wars.wars().len(),5);
            assert_eq!(data.unique_item_count,87);
        }else{};
        let a = parse_df_character_flash_from_file(&test_page)?;
        if let LookupState::FlashCharatcerPage(data) = a {
            assert_eq!(*data.get("Name").unwrap(),"Ruleofnine".to_owned());
            assert_eq!(*data.get("Level").unwrap(),"90");
            assert_ne!(*data.get("Level").unwrap(),"0");
        };
        // parse_df_character_wars_from_file(&test_page)?;
        // parse_df_character_inventory_only_from_file(&test_page)?;
        // parse_df_character_duplicates_from_file(&test_page)?;
        Ok(())
    }
}
