use color_eyre::Result;
use reqwest::Client;
use scraper::Html;
use serde_json::Value;
pub async fn fetch_page(url: &str) -> Result<Html> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status().is_success();
    match status_code {
        true => Ok(Html::parse_document(&response.text().await?)),
        false => Err(color_eyre::eyre::eyre!(format!(
            "Failed to fetch page: HTTP Status Code: {}",
            response.status()
        ))),
    }
}
pub async fn fetch_json(url: &str) -> Result<Value> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let status_code = response.status().is_success();
    match status_code {
        true => {let json: Value = response.json().await?;Ok(json)}
        false => Err(color_eyre::eyre::eyre!(format!(
            "Failed to fetch page: HTTP Status Code: {}",
            response.status()
        ))),
    }
}
pub async fn get_random_event()->String{
    let res = fetch_json("https://history.muffinlabs.com/date").await;
    match res{
        Ok(res) => crate::rng::event_parsing(crate::rng::random_event(res)),
        Err(_) => "None".to_string()
    }
}
