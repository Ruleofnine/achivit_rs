use crate::requests::fetch_page;
use color_eyre::Result;
use scraper::Html;
pub async fn get_df_character(id:u32)->Result<Html>{
    let url = format!("https://account.dragonfable.com/CharPage?id={}",id);
    let response = fetch_page(&url).await?;
    Ok(response)
}
