use color_eyre::Result;
use reqwest::Client;
use scraper::Html;
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
