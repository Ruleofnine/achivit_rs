use color_eyre::Result;
use crate::parsing::*;
use crate::requests::{USER_AGENT,FLASH_USER_AGENT,fetch_page_with_user_agent};
use crate::CHARPAGE;
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
