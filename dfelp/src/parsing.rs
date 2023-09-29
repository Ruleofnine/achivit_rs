use regex::Regex;
use color_eyre::Result;
use std::fs;
use scraper::{Html, Selector};
pub fn convert_html_to_discord_format(input: &str) -> String {
    let re = Regex::new(r#"<a href="(?P<url>[^"]+)"[^>]*>(?P<text>[^<]+)</a>"#).unwrap();
    re.replace_all(input, "[${text}](${url})").to_string()
}
pub fn parse_df_character_from_file(file_path:&str)->Result<()>{
    let data = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let html = Html::parse_document(&data);
    Ok(())

}
