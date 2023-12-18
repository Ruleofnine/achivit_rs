
extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::lookup_df::LookupState;
    use achivit_rs::parsing::{FileFetcher, ParsingCategory, DFCharacterData};
    use color_eyre::Result;
fn extract_data(lookup_state: LookupState) -> Result<DFCharacterData> {
    match lookup_state {
        LookupState::CharacterPage(data) => Ok(data),
        _ => panic!("No Item data for this LookupState"),
    }
}
fn sub_levels(l1:u8,l2:u8)->i8{
        l1 as i8 - l2 as i8
    }
fn sub_items(l1:u16,l2:u16)->i16{
        l1 as i16 - l2 as i16
    }
    #[tokio::test]
    async fn parse_pages() -> Result<()> {
        // let char1 = FileFetcher::new("htmls/mof.html")
        //     .category(ParsingCategory::Compare)
        //     .fetch_data()
        //     .await?
        //     .to_lookupstate()?;
        // let char2 = FileFetcher::new("htmls/3ach.html")
        //     .category(ParsingCategory::Compare)
        //     .fetch_data()
        //     .await?
        //     .to_lookupstate()?;
        // let char1 = extract_data(char1)?; 
        // let char2 = extract_data(char2)?; 
        // let list1 = char1.item_list.unwrap();
        // let list2 = char2.item_list.unwrap();
        // player_two_unique_items.sort();
        // player_one_unique_items.sort();
        // let a = parse_df_character_from_file(&test_page)?;
        // if let LookupState::CharacterPage(data) = a {
        //     assert_eq!(data.name,"Ruleofnine".to_owned());
        //     assert_eq!(data.level,90);
        //     assert_eq!(data.wars.calc_waves_cleared(),10009);
        //     assert_eq!(data.wars.wars().len(),5);
        //     assert_eq!(data.unique_item_count,87);
        // }else{};
        // let a = parse_df_character_flash_from_file(&test_page)?;
        // if let LookupState::FlashCharatcerPage(data) = a {
        //     assert_eq!(*data.get("Name").unwrap(),"Ruleofnine".to_owned());
        //     assert_eq!(*data.get("Level").unwrap(),"90");
        //     assert_ne!(*data.get("Level").unwrap(),"0");
        // };
        // parse_df_character_with_items(&test_page)?;
        // // parse_df_character_inventory_only_from_file(&test_page)?;
        // parse_df_character_duplicates_from_file(&test_page)?;
        Ok(())
    }
}
