
#[allow(unused)]
#[cfg(test)]
mod tests {
    use dfelp::requests::fetch_page;
    use dfelp::lookup::get_df_character;
    use scraper::Selector;
    use dfelp::time::*;
    use super::*;
    #[tokio::test]
    async fn test_lookup() {
        dfelp::parsing::parse_df_character_from_file("../../domp/htmls/2_17.html");
    }
}
