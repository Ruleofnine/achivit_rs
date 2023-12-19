use crate::db::query_with_id;
use crate::embeds::*;
use crate::manage_users::autocomplete_character;
use crate::parsing::{DFCharacterData, WarList, CharacterFetcher, ParsingCategory};
use crate::serenity::Color;
use crate::sheets::compare_sheet;
use crate::{Context, Error};
use color_eyre::{Result,eyre::eyre};
use poise::serenity_prelude::User;
use std::collections::HashMap;
async fn send_embed(state: LookupState, ctx: Context<'_>, df_id: i32) -> Result<()> {
    match state {
        LookupState::NotFound => not_found_embed(ctx, df_id).await?,
        LookupState::CharacterPage(char) => send_character_embed(char, df_id, ctx).await?,
        LookupState::FlashCharatcerPage(char) => {
            send_flash_character_embed(char, df_id, ctx).await?
        }
        LookupState::Wars(name, wars) => send_wars_embed(wars, df_id, name, ctx).await?,
        LookupState::Inventory(name, inventory) => {
            send_inventory_embed(inventory, df_id, name, ctx).await?
        }
        LookupState::Duplicates(name, dups) => {
            send_duplicates_embed(dups, df_id, name, ctx).await?
        }
    };
    Ok(())
}
#[derive(Debug)]
pub enum LookupState {
    FlashCharatcerPage(HashMap<String, String>),
    CharacterPage(DFCharacterData),
    Inventory(String, Vec<String>),
    Wars(String, WarList),
    Duplicates(String, HashMap<String, i32>),
    NotFound,
}
#[allow(non_camel_case_types)]
#[derive(poise::ChoiceParameter, PartialEq)]
pub enum LookupCategory {
    CharacterPage,
    FlashCharacterPage,
    Inventory,
    Wars,
    Duplicates,
}
impl LookupState{
   pub fn extract_data(lookup_state: LookupState) -> Result<DFCharacterData> {
        match lookup_state {
            LookupState::CharacterPage(data) => Ok(data),
            _ => Err(eyre!("No Item data for this LookupState")),
        }
    }    }

/// Lookup a DF Character in various ways
#[poise::command(slash_command)]
pub async fn lookup_df_character(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
    category: Option<LookupCategory>,
) -> Result<(), Error> {
    let pool = &ctx.data().db_connection;
    let category = category.unwrap_or(LookupCategory::CharacterPage);
    let df_id = match (character, user) {
        (Some(character), _) => Some(character),
        (None, None) => query_with_id(pool, ctx.author().id.0).await?,
        (None, Some(user)) => query_with_id(pool, user.id.0).await?,
    };
    let df_id = match df_id {
        Some(df_id) => df_id,
        None => {
            ctx.send( |f| {
                    f.embed(|f| {
                        f.title("No Characters Registered")
                            .color(Color::DARK_RED)
                            .description("You have no DF Characters registered, ask an administrator to regsiter your df id(s)")
                    })
                })
                .await?;
            return Ok(());
        }
    };

    let lookupstate = CharacterFetcher::new(df_id,category)
        .fetch_data()
        .await?.to_lookupstate()?;
    Ok(send_embed(lookupstate, ctx, df_id).await?)
}
/// Compare two DF Characters in various ways
#[poise::command(slash_command)]
pub async fn compare_df_characters(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character1: i32,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character2: i32,
) -> Result<(), Error> {
    let main_state = CharacterFetcher::new(character1,LookupCategory::CharacterPage)
        .category(ParsingCategory::Items)
        .fetch_data()
        .await?.to_lookupstate()?;
    let second_state = CharacterFetcher::new(character2,LookupCategory::CharacterPage)
        .category(ParsingCategory::Items)
        .fetch_data()
        .await?.to_lookupstate()?;
    let mut not_found:Vec<i32> = vec![];
    match (&main_state,&second_state){
        (LookupState::NotFound,LookupState::NotFound)=>{not_found.extend(vec![character1,character2]);},
        (LookupState::NotFound,_) =>{not_found.push(character1);},
        (_,LookupState::NotFound) =>{not_found.push(character2)},
        _ => ()
    };
    match not_found.len(){
        0 => {},
        _ => {compare_not_found_embed(ctx, not_found).await?}

    };
    let sheet_data = compare_sheet(main_state, second_state).await?;
    match sheet_data{
        Some(sheet) =>send_compare_embed(sheet,ctx).await?,
        None => ()
    };
    Ok(())
}
/// Compare two DF Characters in various ways
#[poise::command(slash_command)]
pub async fn roles_list(ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Not used in Guild");
    Ok(())
}
