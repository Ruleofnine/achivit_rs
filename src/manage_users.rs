use crate::{Context, Error};
use color_eyre::Result;
use dfelp::lookup::get_df_character;
use dfelp::CHARPAGE;
use poise::serenity_prelude::User;
use regex::Regex;
use serenity::utils::Color;
use sqlx::query;
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn register_character(ctx: Context<'_>, mut user: User, df_id: i32) -> Result<(), Error> {
    let pool = &ctx.data().db_connection;
    let author = &ctx.author().name;
    let mut user_id = user.id.0 as i64;
    query!("INSERT INTO users (discord_id,discord_name,registered_by) VALUES ($1,$2,$3) ON CONFLICT (discord_id) DO NOTHING",user_id,user.name,author).execute(pool).await?;
    let result = get_df_character(df_id).await?;
    let character = match result {
        Some(char) => char,
        None => {
            poise::send_reply(ctx, |f| {
                f.embed(|f| {
                    f.title(format!("No Character with DF ID: [{}]", df_id))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description("The game character you are searching for does not exist.")
                        .image("https://account.dragonfable.com/images/bgs/bg-df-main.jpg")
                })
            })
            .await?;
            return Ok(());
        }
    };
    let res = query!("INSERT INTO df_characters (discord_id,df_id,character_name,registered_by) VALUES ($1,$2,$3,$4) ON CONFLICT (df_id) DO NOTHING",user_id,df_id,character.name,author).execute(pool).await?;
    let color: Color;
    let title: String;
    let username: String;
    if res.rows_affected() == 0 {
        color = Color::DARK_RED;
        title = format!("Already Registered: {}", character.name);
        user_id = query!(
            "SELECT discord_id FROM df_characters WHERE df_id = $1",
            df_id
        )
        .fetch_one(pool)
        .await?
        .discord_id;
        user = ctx.http().get_user(user_id as u64).await?;
    } else {
        color = Color::DARK_GOLD;
        title = format!("Sucessfully Registered: {}", character.name);
    };
    username = user.name.to_owned();
    let icon_url = user.face();
    let chars_list = query!(
        "SELECT df_id,character_name FROM df_characters WHERE discord_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let chars_string = chars_list
        .iter()
        .map(|c| {
            format!(
                "**DF ID:** {} [{}]({}{})",
                c.df_id, c.character_name, CHARPAGE, c.df_id
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    poise::send_reply(ctx, |f| {
        f.embed(|f| {
            f.title(title)
                .url(format!("{}{}", CHARPAGE, df_id))
                .color(color)
                .author(|a| a.name(&username).icon_url(icon_url))
                .description(chars_string)
        })
    })
    .await?;
    Ok(())
}

fn extract_name_from_invokation_data(input: &str) -> i64 {
    let re = Regex::new(r"user:(\d+)").unwrap();
    // Search for the regex pattern in the input string
    if let Some(captures) = re.captures(input) {
        if let Some(number) = captures.get(1) {
            let user_id: i64 = number.as_str().parse().unwrap();
            return user_id;
        }
    }
    0
}

pub async fn autocomplete_character(
    ctx: Context<'_>,
    partial: &str,
) -> Vec<poise::AutocompleteChoice<i32>> {
    dbg!("test");
    let discord_id = extract_name_from_invokation_data(&ctx.invocation_string());
    let mut ac_choices: Vec<poise::AutocompleteChoice<i32>> = Vec::new();
let query = match partial.parse::<usize>() {
    Ok(num) => format!("SELECT df_id, character_name FROM df_characters WHERE discord_id = $1 AND CAST(df_id AS TEXT) LIKE '%{}%' ORDER BY created ASC",num),
    Err(_) => format!("SELECT df_id, character_name FROM df_characters WHERE discord_id = $1 AND character_name ILIKE '%{}%' ORDER BY created ASC",partial)
};
let res = sqlx::query_as::<_,CharacterQuery>(&query)
    .bind(discord_id)
    .bind(partial)
    .fetch_all(&ctx.data().db_connection)
    .await;
    let chars = match res{
        Ok(chars) => chars,
        Err(_)=>return ac_choices
    };
    ac_choices = chars
        .iter()
        .map(|choice| poise::AutocompleteChoice {
            name: format!("{}: {}", choice.character_name, choice.df_id),
            value: choice.df_id,
        })
        .collect();
    ac_choices

}

#[derive(sqlx::FromRow)]
struct CharacterQuery{
    character_name:String,
    df_id:i32
}
