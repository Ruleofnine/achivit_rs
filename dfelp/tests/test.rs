
#[allow(unused)]
#[cfg(test)]
mod tests {
    use soup::Soup;
    use dfelp::requests::fetch_page;
    use dfelp::lookup::get_df_character;
    use dfelp::time::*;
    use std::fs;
    #[tokio::test]
    async fn test_lookup() {
        // assert!(dfelp::parsing::parse_df_character_from_file("../../domp/htmls/1_18.html").is_err(),"Wasn't an Error!");
        // dfelp::parsing::parse_df_character_from_file("../../domp/htmls/2_17.html");
        // let character = dfelp::parsing::parse_df_character_from_file("../../domp/htmls/20302406_16.html").expect("failed to get character").unwrap();
        // assert_eq!(character.unique_item_count,1066);
        // let character = get_df_character(20302406).await.unwrap().unwrap();
        // assert_eq!(character.unique_item_count,1066);
        let data = fs::read_to_string("../../domp/htmls/20302406_16.html").expect("failed to get file");
        let document = Soup::new(&data);
       dbg!(dfelp::parsing::parse_df_character_wars_only(document).await);
    }
    #[tokio::test]
    async fn test_lookup_scrap(){
        let character = get_df_character(4211037).await.expect("failed to get character").unwrap();
        dbg!(&character);
        assert_eq!(character.level,90)
    }
}
