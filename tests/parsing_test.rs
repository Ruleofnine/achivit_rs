extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::roles::*;
    use achivit_rs::{
        lookup_df::LookupState,
        parsing::{DFCharacterData, FileFetcher, ParsingCategory},
    };
    use color_eyre::Result;
    fn extract_data(lookup_state: LookupState) -> Result<DFCharacterData> {
        match lookup_state {
            LookupState::CharacterPage(data) => Ok(data),
            _ => panic!("No Item data for this LookupState"),
        }
    }
    #[tokio::test]
    async fn parse_pages() -> Result<()> {
        let roles = get_roles("JSONS/roles.json")?;
        let char = FileFetcher::new("htmls/ruleofnine.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?;
        let mut char = extract_data(char)?;
        let char_items = char.item_list.take().unwrap().all();
        let mut aquired_roles: Vec<&Role> = Vec::new();
        for role in roles.roles() {
            let mut aquired = false;
            match role.req_type {
                ReqType::Wars => {
                    let amount = role.amount.unwrap_or_default() as usize;
                    if &char.wars.wars().len() >= &amount {
                        aquired = true;
                    }
                }
                ReqType::Item => {
                    let items = role
                        .required
                        .as_ref()
                        .expect("Item Role requires item list");
                    aquired = items.iter().all(|i| char_items.contains(i));
                }
                ReqType::ItemsAmount => {
                    let amount = role.amount().expect("Items/Amount Needs amount.");
                    let items = role
                        .required
                        .as_ref()
                        .expect("Item Role requires item list");
                    let count = items.iter().filter(|&i| char_items.contains(i)).count();
                    if count as i32 >= amount {
                        aquired = true
                    }
                }
                ReqType::Waves => {
                    let amount = role.amount().expect("Waves Needs amount.");
                    for war in char.wars().war_list() {
                        if &war.waves_int() >= &amount {
                            aquired = true;
                            break;
                        }
                    }
                }
                ReqType::Gold => {
                    let amount = role.amount().expect("gold Needs amount.");
                    if char.gold() >= &amount {
                        aquired = true;
                    }
                }
                ReqType::Max => {
                    let prereq_roles: &Vec<String> =
                        role.prereqs().as_ref().expect("MAX role needs prereqs");
                    if prereq_roles
                        .iter()
                        .all(|p| aquired_roles.iter().any(|r| r.name() == p))
                    {
                        aquired = true
                    };
                }
            }
            if aquired {
                aquired_roles.push(role)
            }
        }
        let mut roles_clone = aquired_roles.clone();
        for role in aquired_roles {
            match role.prereqs() {
                Some(reqs) => for req in reqs {
                    roles_clone.retain(|r|&r.name!=req)
                },
                None => (),
            }
        }
        dbg!(&roles_clone);
        dbg!(roles_clone.len());
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
