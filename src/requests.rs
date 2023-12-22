use color_eyre::eyre::eyre;
use color_eyre::Result;
use log::{error,info};
use reqwest::{Client, StatusCode};
use serde_json::Value;
pub const FLASH_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ArtixGameLauncher/2.0.7 Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.137 Electron/8.1.0 Safari/537.36";
pub const CHARPAGE: &str = "https://account.dragonfable.com/CharPage?id=";
pub const DA_IMGUR: &str = "https://i.imgur.com/70CNN80.png";
pub const NDA_IMGUR: &str = "https://i.imgur.com/rBJt81B.png";
pub const ROLE_DA_IMGUR: &str = "https://i.imgur.com/uxK6enN.png";
pub const ASCEND_DA_IMGUR: &str = "https://i.imgur.com/MHJCKwE.png";
pub const COLOR_SITE: &str = "https://www.color-hex.com/color/";
pub async fn fetch_page_with_user_agent(user_agent: &str, url: &str) -> Result<String> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    let response = client.get(url).send().await?;
    info!("GET Request -> {url} STATUS : {}",response.status());
    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(response.text().await?),
        StatusCode::TOO_MANY_REQUESTS => {
            error!("TOO_MANY_REQUESTS");
            Err(eyre!(status_code))
        }
        other => {
            error!("{}", other);
            Err(eyre!(other))
        }
    }
}
pub async fn fetch_json(url: &str) -> Result<Value> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status();
    info!("GET Request -> {url} STATUS : {}",response.status());
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
