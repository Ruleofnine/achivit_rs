use crate::manage_users::autocomplete_character;
use crate::{Context, Error};
use color_eyre::Result;
use poise::serenity_prelude::User;
use serde::Serialize;

#[derive(poise::ChoiceParameter, PartialEq, Debug,Serialize)]
pub enum ChallengeCategory {
    Gold,
    Waves,
    Item,
}
/*
pub async fn autocomplete_category(
    ctx: Context<'_>,
    partial: &str,
    other:Option<Vec<String>>,
) -> Vec<poise::AutocompleteChoice<String>> {
    let ac_choices: Vec<poise::AutocompleteChoice<String>> = Vec::new();
    if partial.is_empty() {
        let ac_choices: Vec<poise::AutocompleteChoice<String>> =
            vec![ChallengeCategory::Gold, ChallengeCategory::Waves];
        return ac_choices;
    }
    ac_choices
}

/// Lookup a DF Character in various ways
#[poise::command(slash_command)]
pub async fn challenge_user(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
    #[autocomplete = "autocomplete_category"] challenge_category: ChallengeCategory,
) -> Result<(), Error> {
    Ok(())
}
*/
