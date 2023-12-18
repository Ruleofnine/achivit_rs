extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::roles::*;
    use achivit_rs::{
        lookup_df::LookupState,
        parsing::{DFCharacterData, FileFetcher, ParsingCategory, WarList},
    };
    use color_eyre::Result;
    use std::collections::{BTreeSet, HashSet};
    fn extract_data(lookup_state: LookupState) -> Result<DFCharacterData> {
        match lookup_state {
            LookupState::CharacterPage(data) => Ok(data),
            _ => panic!("No Item data for this LookupState"),
        }
    }
    fn check_item(role: &Role, char_items: &BTreeSet<String>) -> bool {
        let items = role
            .required
            .as_ref()
            .expect("Item Role requires item list");
        items.iter().all(|i| char_items.contains(i))
    }
    fn check_war(role: &Role, char: &DFCharacterData) -> bool {
        let amount = role.amount.expect("War needs amount") as usize;
        char.wars.wars().len() >= amount
    }
    fn check_item_amount(role: &Role, char_items: &BTreeSet<String>) -> bool {
        let amount = role.amount().expect("Items/Amount Needs amount.");
        let items = role
            .required
            .as_ref()
            .expect("Item Role requires item list");
        let count = items.iter().filter(|&i| char_items.contains(i)).count();
        count as i32 >= amount
    }
    fn check_waves(role: &Role, wars: &WarList) -> bool {
        let amount = role.amount().expect("Waves Needs amount.");
        wars.war_list().iter().any(|w| w.waves_int() >= amount)
    }
    fn check_gold(role: &Role, gold: &i32) -> bool {
        let amount = role.amount().expect("gold Needs amount.");
        gold >= &amount
    }
    fn check_max_role(role: &Role, aquired_roles: &Vec<&Role>) -> bool {
        let prereq_roles: &Vec<String> = role.prereqs().as_ref().expect("MAX role needs prereqs");
        prereq_roles
            .iter()
            .all(|p| aquired_roles.iter().any(|r| r.name() == p))
    }

    #[tokio::test]
    async fn parse_pages() -> Result<()> {
        let roles = get_roles("JSONS/roles.json")?;
        let char = FileFetcher::new("htmls/3ach.html")
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?;
        let mut char = extract_data(char)?;
        let char_items = char.item_list.take().unwrap().all();
        let mut aquired_roles: Vec<&Role> = Vec::new();
        for role in roles.roles() {
            let aquired = match role.req_type {
                ReqType::Wars => check_war(role, &char),
                ReqType::Item => check_item(role, &char_items),
                ReqType::ItemsAmount => check_item_amount(role, &char_items),
                ReqType::Waves => check_waves(role, &char.wars),
                ReqType::Gold => check_gold(role, &char.gold()),
                ReqType::Max => check_max_role(role, &aquired_roles),
            };
            if aquired {
                aquired_roles.push(role)
            }
        }
        let mut roles_to_remove = HashSet::new();
        for role in &aquired_roles {
            if let Some(reqs) = role.prereqs() {
                for req in reqs {
                    if aquired_roles.iter().any(|r| r.name() == req) {
                        roles_to_remove.insert(req.clone());
                    }
                }
            }
        }
        aquired_roles.retain(|r| !roles_to_remove.contains(r.name()));
        dbg!(aquired_roles.len());
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
