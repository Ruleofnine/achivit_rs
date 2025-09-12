use std::collections::HashMap;
use std::vec;

use crate::db::query_with_id;
use crate::lookup_df::LookupCategory;
use crate::manage_users::autocomplete_character;
use crate::parsing::{CharacterFetcher, ParsingCategory};
use crate::requests::{fetch_page_with_user_agent, CHARPAGE};
use crate::{Context, Error};
use chrono::Duration;
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use poise::serenity_prelude::User;
use poise::AutocompleteChoice;
use reqwest::header::USER_AGENT;
use serde::Serialize;

fn score_suggestion(suggestion: &str, partial: &str) -> i32 {
    if suggestion.eq_ignore_ascii_case(partial) {
        0 // Exact match
    } else if suggestion
        .to_lowercase()
        .starts_with(&partial.to_lowercase())
    {
        1 // Prefix match
    } else if suggestion.to_lowercase().contains(&partial.to_lowercase()) {
        2 // Substring match
    } else {
        3 // No match
    }
}
fn sort_suggestions(suggestions: &mut Vec<String>, partial: &str) {
    suggestions.sort_by(|a, b| {
        // Example scoring logic:
        // - Exact matches first
        // - Prefix matches next
        // - Substring matches last
        // - Alphabetical order as a tiebreaker

        let a_score = score_suggestion(a, partial);
        let b_score = score_suggestion(b, partial);

        // Sort by score (ascending)
        a_score.cmp(&b_score).then_with(|| a.cmp(b))
    });
}
#[derive(poise::ChoiceParameter, PartialEq, Debug, Serialize)]
pub enum ChallengeCategory {
    Gold,
    Waves,
    Item,
}
pub async fn autocomplete_category(
    ctx: Context<'_>,
    partial: &str,
) -> impl Iterator<Item = String> {
    let mut cache = ctx.data().item_cache.lock().await;
    let pool = &ctx.data().db_connection;
    if let Some(suggestions) = cache.get(ctx.author().id.as_u64()) {
        if partial.is_empty() {
            return suggestions.clone().into_iter();
        } else {
            let mut suggestions = suggestions.clone();
            sort_suggestions(&mut suggestions, partial);
            return suggestions.into_iter();
        }
    }

    if let Ok(Some(df_id)) = query_with_id(pool, ctx.author().id.0).await {
        if let Ok(lookupstate) = CharacterFetcher::new(df_id, LookupCategory::CharacterPage)
            .category(ParsingCategory::Items)
            .fetch_data()
            .await
            .and_then(|data| data.to_lookupstate())
        {
            if let Ok(chardata) = lookupstate.extract_character_data() {
                let mut keys: Vec<String> = chardata
                    .take_items()
                    .unwrap()
                    .items()
                    .keys()
                    .map(|k| k.to_string())
                    .collect();
                keys.insert(0, "Waves".to_owned());
                keys.insert(0, "Gold".to_owned());
                cache.insert(*ctx.author().id.as_u64(), keys.clone());
                return keys.into_iter();
            }
        }
    }
    // Default empty iterator if no data is found
    Vec::new().into_iter()
}

pub async fn autocomplete_duration(
    _ctx: Context<'_>,
    partial: &str,
) -> Vec<AutocompleteChoice<i64>> {
    if let Ok(iminutes) = partial.parse::<i64>() {
        let time = chrono::Local::now();
        let minutes = Duration::minutes(iminutes);
        if let Some(end_time) = time.checked_add_signed(minutes) {
            let time_str = end_time.format("%B %d, %Y %I:%M %p EST").to_string();
            let choice = AutocompleteChoice {
                name: time_str,
                value: iminutes,
            };
            return vec![choice];
        }
    }
    let mut default_choices = HashMap::new();
    default_choices.insert("10 Minutes", 10);
    default_choices.insert("30 Minutes", 30);
    default_choices.insert("1 Hour", 60);
    default_choices.insert("2 Hours", 120);
    default_choices.insert("6 Hours", 360);
    default_choices.insert("12 Hours", 720);
    default_choices.insert("1 Day", 1440);
    default_choices.insert("7 Days", 10080);
    return default_choices
        .into_iter()
        .map(|(key, value)| AutocompleteChoice {
            name: key.to_owned(),
            value,
        })
        .collect();
}

/// Challenge a user to a contest in DF
#[poise::command(slash_command)]
pub async fn challenge_user(
    ctx: Context<'_>,
    #[description = "User to challenge"] user: User,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: i32,
    #[description = "Gold/Waves/Item you want to challenge user to"]
    #[autocomplete = "autocomplete_category"]
    challenge_category: String,
    #[description = "Duration in minutes"]
    #[autocomplete = "autocomplete_duration"]
    duration: i64,
) -> Result<(), Error> {
    ctx.reply(format!(
        "You challenge {} with their character of {character} to [{challenge_category}] end date {}",
        user.id,duration
    ))
    .await?;
    Ok(())
}
