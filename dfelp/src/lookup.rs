use color_eyre::Result;
use crate::parsing::{DFCharacterData, parse_df_character};
use crate::requests::{USER_AGENT,fetch_page_with_user_agent};
use crate::CHARPAGE;
pub async fn get_df_character(id:i32)->Result<Option<DFCharacterData>>{
    let url = format!("{}{}",CHARPAGE,id);
    let response = fetch_page_with_user_agent(USER_AGENT,&url).await?;
    Ok(parse_df_character(response))
}
