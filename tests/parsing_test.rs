extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::lookup_df::LookupState;
    use achivit_rs::parsing::{ParsingCategory,FileFetcher};
    use achivit_rs::db::{get_env_info,create_db,initialize_db};
    use color_eyre::Result;

    #[tokio::test]
    async fn test() -> Result<()> {
        let char = FileFetcher::new("htmls/ruleofnine.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?;
        let data = LookupState::extract_data(char)?;
        assert_eq!(&data.name , "Ruleofnine");
        assert_eq!(*data.level() , 90);
        // let inn_reqs = get_InnList()?;
        // let reqs = inn_reqs.reqs();
        // for (_,i) in reqs{
        //     for y in i.reqs() {
        //         let bool =data.item_list().as_ref().unwrap().contains(y);
        //         match bool{
        //             true => (),
        //             false => {dbg!(y);}
        //         }
        //         
        //     }
        // }
        // reqs.iter().all(|req|req.1.reqs().iter().all(|i|data.item_list());
        // let roles = check_roles(data,"ascendancies.json")?;       // let char1 = extract_data(char1)?;
        // dbg!(roles.roles());
        // dbg!(roles.roles().len());
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
#[tokio::test]
async fn generate_db_init()->Result<()>{
    dotenv::dotenv().ok();
    Ok(())
}
}

