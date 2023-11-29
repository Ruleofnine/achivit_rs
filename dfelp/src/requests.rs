use color_eyre::eyre::eyre;
use color_eyre::Result;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use soup::Soup;
pub const FLASH_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ArtixGameLauncher/2.0.7 Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub async fn fetch_page(url: &str) -> Result<Soup> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(Soup::new(&response.text().await?)),
        other => Err(eyre!(other)),
    }
}
pub async fn fetch_page_with_user_agent(user_agent:&str,url:&str)->Result<Soup>{
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    let response = client.get(url).send().await?;
    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(Soup::new(&response.text().await?)),
        other => Err(eyre!(other)),
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
