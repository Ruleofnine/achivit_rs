use crate::db::query_with_id;
use crate::embeds::*;
use crate::manage_users::autocomplete_character;
use crate::parsing::{DFCharacterData, WarList,HttpFetcher,ParsingCategory,DataFetcher};
use crate::serenity::Color;
use crate::{Context, Error};
use color_eyre::Result;
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
        LookupState::Compare => (),
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
    Compare,
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

    let lookupstate = HttpFetcher::new_character_page(df_id).fetch_and_parse_data(category.into()).await?;
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
    let pool = &ctx.data().db_connection;
    // let char1 = get_df_character(character1).await?;
    // let char1 = get_df_character(character2).await?;
    // lookupcommand.state(lookup(&lookupcommand.category, df_id).await?);
    // Ok(send_embed(lookupcommand.state, ctx, df_id).await?)
    Ok(())
}
