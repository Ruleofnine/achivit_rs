
#[allow(unused)]
#[cfg(test)]
mod tests {
    use dfelp::requests::{fetch_page, FLASH_USER_AGENT, fetch_page_with_user_agent};
    use dfelp::parsing::*;
    use dfelp::lookup::{get_df_character,get_df_character_duplicates, get_df_character_flash};
    use dfelp::parsing::parse_df_character_wars_from_file;
    use dfelp::{time::*, CHARPAGE};
    use std::fs;
    use scraper::Html;
    #[tokio::test]
    async fn test_lookup() {
        // assert!(dfelp::parsing::parse_df_character_from_file("../../domp/htmls/1_18.html").is_err(),"Wasn't an Error!");
        // let mof =dfelp::parsing::parse_df_character_from_file("../../domp/htmls/mof.html").unwrap().expect("nsheothn");
        // let test = get_df_character_flash(22).await;
        // dbg!(test.uf);
        let character = dfelp::parsing::parse_df_character_flash_from_file("../../domp/htmls/flash_ruleofnine.html").expect("failed to get character").unwrap();
        dbg!(character);
        // dbg!(get_discord_embed_description_flash(character,22));
        // assert_eq!(&character.dragon.as_ref().unwrap().name,"Hyonix");
        // assert_eq!(&character.dragon.as_ref().unwrap().dragon_type,"Ice");
        // assert_eq!(character.unique_item_count,1066);
        // let character = dfelp::parsing::parse_df_character_from_file("../../domp/htmls/fake_dragon.html").expect("failed to get character").unwrap();
        // assert_eq!(character.dragon.as_ref().unwrap().name,"8====D (fake dragon)");
        // assert_eq!(character.dragon.as_ref().unwrap().dragon_type,"Ice");
       //  parse_df_character_inventory_only_from_file("../../domp/htmls/2_17.html").expect("failed").unwrap();
       // // parse_df_character_inventory_only_from_file("../../domp/htmls/mof.html").expect("failed").unwrap();
       // dbg!(parse_df_character_duplicates_from_file("../../domp/htmls/mof.html").expect("failed").unwrap());
        // let data = fs::read_to_string("../../domp/htmls/20302406_16.html").expect("failed to get file");

    }
    #[tokio::test]
    async fn test_lookup_scrap(){
        // let character = get_df_character_duplicates(4211037).await.expect("failed to get character").unwrap();
        // dbg!(&character);
        // assert_eq!(character.level,90)
    }
}
