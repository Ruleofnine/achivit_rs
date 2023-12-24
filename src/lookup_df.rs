use crate::db::query_with_id;
use crate::embeds::*;
use crate::guild_settings::GuildSettings;
use crate::manage_users::autocomplete_character;
use crate::parsing::{CharacterFetcher, DFCharacterData, ParsingCategory, WarList};
use crate::requirements::{get_requirements, RequirementListType};
use crate::serenity::Color;
use crate::sheets::compare_sheet;
use crate::{Context, Error};
use color_eyre::{eyre::eyre, Result};
use poise::serenity_prelude::User;
use std::collections::HashMap;
use std::fmt::Display;

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
        LookupState::Roles(char) => send_roles_embed(df_id,char,ctx,RequirementListType::Roles).await?,
        LookupState::Ascendancies(char) => send_roles_embed(df_id,char,ctx,RequirementListType::Ascend).await?
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
    Roles(DFCharacterData),
    Ascendancies(DFCharacterData),
    NotFound,
}
#[allow(non_camel_case_types)]
#[derive(poise::ChoiceParameter, PartialEq,Debug)]
pub enum LookupCategory {
    CharacterPage,
    FlashCharacterPage,
    Inventory,
    Wars,
    Duplicates,
    Roles,
    Ascendancies,
}
impl Display for LookupState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lookup_type = match self{
            LookupState::Inventory(_,_)=>"Inventory",
            LookupState::CharacterPage(_)=>"CharacterPage",
            LookupState::Wars(_,_)=>"Wars",
            LookupState::Roles(_)=>"Roles",
            LookupState::Duplicates(_,_ )=>"Duplicates",
            LookupState::Ascendancies(_)=>"Ascendancies",
            LookupState::FlashCharatcerPage(_)=>"FlashCharacterPage",
            LookupState::NotFound=>"NotFound"
        };
        write!(f, "{}",lookup_type)
    }

}
impl LookupState {
    
    pub fn extract_character_data(self) -> Result<DFCharacterData> {
        match self {
            LookupState::CharacterPage(data) => Ok(data),
            LookupState::Ascendancies(data) => Ok(data),
            LookupState::Roles(data) => Ok(data),
            _ => Err(eyre!("Incorrect LookupState: {}",self)),
        }
    }
    pub fn extract_inventory_data(self) -> Result<(String,Vec<String>)> {
        match self {
            LookupState::Inventory(name,items) => Ok((name,items)),
            _ => Err(eyre!("Incorrect LookupState :{}",self)),
        }
    }
}

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
    let lookupstate = CharacterFetcher::new(df_id, category)
        .fetch_data()
        .await?
        .to_lookupstate()?;
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
    let main_state = CharacterFetcher::new(character1, LookupCategory::CharacterPage)
        .category(ParsingCategory::Items)
        .fetch_data()
        .await?
        .to_lookupstate()?;
    let second_state = CharacterFetcher::new(character2, LookupCategory::CharacterPage)
        .category(ParsingCategory::Items)
        .fetch_data()
        .await?
        .to_lookupstate()?;
    let mut not_found: Vec<i32> = vec![];
    match (&main_state, &second_state) {
        (LookupState::NotFound, LookupState::NotFound) => {
            not_found.extend(vec![character1, character2]);
        }
        (LookupState::NotFound, _) => {
            not_found.push(character1);
        }
        (_, LookupState::NotFound) => not_found.push(character2),
        _ => (),
    };
    match not_found.len() {
        0 => {}
        _ => compare_not_found_embed(ctx, not_found).await?,
    };
    let sheet_data = compare_sheet(main_state, second_state).await?;
    if let Some(sheet) = sheet_data { send_compare_embed(sheet, ctx).await? }
    Ok(())
}
/// Get the DF role list for this server
#[poise::command(slash_command)]
pub async fn roles_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("").0 as i64;
    let pool = &ctx.data().db_connection;
    let settings =
        sqlx::query_as::<_, GuildSettings>("select * from guild_settings where guild_id = $1")
            .bind(guild_id)
            .fetch_optional(pool)
            .await?;
    let guild_settings = match settings {
        None => return Ok(no_settings_embed(ctx).await?),
        Some(settings) => settings,
    };
    let mut roles = get_requirements(guild_settings.roles_path())?;
    Ok(roles_embed(ctx, &mut roles).await?)
}

