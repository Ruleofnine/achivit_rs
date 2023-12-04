use crate::embeds::*;
use crate::manage_users::autocomplete_character;
use crate::parsing::{DFCharacterData, WarList};
use crate::requests::*;
use crate::serenity::Color;
use crate::{Context, Error};
use color_eyre::Result;
use poise::serenity_prelude::User;
use sqlx::{query, PgPool};
use std::collections::HashMap;
struct LookUpCommand {
    state: LookupState,
    category: LookupCategory,
}
async fn lookup(category: &LookupCategory, df_id: i32) -> Result<LookupState> {
    Ok(match category {
        LookupCategory::CharacterPage => get_df_character(df_id).await?,
        LookupCategory::FlashCharacterPage => get_df_character_flash(df_id).await?,
        LookupCategory::Inventory => get_df_character_inventory_only(df_id).await?,
        LookupCategory::Wars => get_df_character_wars_only(df_id).await?,
        LookupCategory::Duplicates => get_df_character_duplicates(df_id).await?,
    })
}
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

impl LookUpCommand {
    fn new(category: Option<LookupCategory>) -> LookUpCommand {
        let category = match category {
            None => LookupCategory::CharacterPage,
            Some(cat) => cat,
        };
        LookUpCommand {
            state: LookupState::NotFound,
            category,
        }
    }
    fn state(&mut self,state:LookupState){
        self.state = state;
    }
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
enum LookupCategory {
    CharacterPage,
    FlashCharacterPage,
    Inventory,
    Wars,
    Duplicates,
}
impl LookUpCommand {}

async fn query_with_id(pool: &PgPool, id: u64) -> Option<i32> {
    match query!(
        "select df_id from df_characters where discord_id = $1 order by created asc limit 1",
        id as i64
    )
    .fetch_one(pool)
    .await
    {
        Ok(num) => Some(num.df_id),
        Err(_) => None,
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
    let mut lookupcommand = LookUpCommand::new(category);
    let pool = &ctx.data().db_connection;

    let df_id = match (character, user) {
        (Some(character), _) => Some(character),
        (None, None) => query_with_id(pool, ctx.author().id.0).await,
        (None, Some(user)) => query_with_id(pool, user.id.0).await,
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
    lookupcommand.state(lookup(&lookupcommand.category, df_id).await?);
    Ok(send_embed(lookupcommand.state, ctx, df_id).await?)
}
