use color_eyre::eyre::eyre;
use log::error;
use color_eyre::Result;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use scraper::Html;
use crate::lookup_df::LookupState;
use crate::parsing::*;
pub const FLASH_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ArtixGameLauncher/2.0.7 Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub const CHARPAGE: &str = "https://account.dragonfable.com/CharPage?id=";
pub const DA_IMGUR: &str = "https://i.imgur.com/70CNN80.png";
pub const NDA_IMGUR: &str = "https://i.imgur.com/rBJt81B.png";
pub const COLOR_SITE: &str = "https://www.color-hex.com/color/";
pub async fn get_df_character(id:i32)->Result<LookupState>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(USER_AGENT,&url).await?;
    Ok(parse_df_character(response))
}
pub async fn get_df_character_flash(id:i32)->Result<LookupState>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(FLASH_USER_AGENT,&url).await?;
    Ok(parse_df_character_flash(response))
}
pub async fn get_df_character_wars_only(id:i32)->Result<LookupState>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(USER_AGENT,&url).await?;
    Ok(parse_df_character_wars_only(response))
}
pub async fn get_df_character_inventory_only(id:i32)->Result<LookupState>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(USER_AGENT,&url).await?;
    Ok(parse_df_character_inventory_only(response))
}
pub async fn get_df_character_duplicates(id:i32)->Result<LookupState>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(USER_AGENT,&url).await?;
    Ok(parse_df_character_duplicates(response))
}
pub async fn fetch_page(url: &str) -> Result<Html> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(Html::parse_document(&response.text().await?)),
        other => {error!("{}",other);
            Err(eyre!(other))},
    }
}
pub async fn fetch_page_with_user_agent(user_agent:&str,url:&str)->Result<Html>{
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    let response = client.get(url).send().await?;
    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(Html::parse_document(&response.text().await?)),
        StatusCode::TOO_MANY_REQUESTS => {error!("TOO_MANY_REQUESTS");Err(eyre!(status_code))}
        other => {error!("{}",other);
            Err(eyre!(other))},
    }
}
pub async fn fetch_json(url: &str) -> Result<Value> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status();
    match status_code {
        StatusCode::OK => {
            let json: Value = response.json().await?;
            Ok(json)
        }
        other => Err(eyre!(other)),
    }
}
pub async fn get_random_event() -> String {
    let res = fetch_json("https://history.muffinlabs.com/date").await;
    match res {
        Ok(res) => crate::rng::event_parsing(crate::rng::random_event(res)),
        Err(_) => "None".to_string(),
    }
}
