use crate::embeds::not_found_embed;
use crate::lookup_df::LookupState;
use crate::requests::{get_df_character, CHARPAGE};
use crate::{Context, Error};
use color_eyre::Result;
use poise::serenity_prelude::User;
use regex::Regex;
use serenity::utils::Color;
use sqlx::query;
/// Register Character by ID
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn register_character(ctx: Context<'_>, mut user: User, df_id: i32) -> Result<(), Error> {
    let pool = &ctx.data().db_connection;
    let author = &ctx.author().name;
    let mut user_id = user.id.0 as i64;
    query!("INSERT INTO users (discord_id,discord_name,registered_by) VALUES ($1,$2,$3) ON CONFLICT (discord_id) DO NOTHING",user_id,user.name,author).execute(pool).await?;
    let result = get_df_character(df_id).await?;
    let character = match result {
        LookupState::CharacterPage(char) => char.name,
        LookupState::FlashCharatcerPage(char) => char.get("Name").take().unwrap().to_owned(),
        LookupState::NotFound => return Ok(not_found_embed(ctx, df_id).await?),
        _ => panic!("Unexpected LookupState",),
    };
    dbg!(&character);
    let res = query!("INSERT INTO df_characters (discord_id,df_id,character_name,registered_by) VALUES ($1,$2,$3,$4) ON CONFLICT (df_id) DO NOTHING",user_id,df_id,character,author).execute(pool).await?;
    let color: Color;
    let title: String;
    let username: String;
    if res.rows_affected() == 0 {
        color = Color::DARK_RED;
        title = format!("Already Registered: {}", character);
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
        title = format!("Sucessfully Registered: {}", character);
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
    dbg!(&character);

    ctx.send(|f| {
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
    if let Some(captures) = re.captures(input) {
        if let Some(number) = captures.get(1) {
            let user_id: i64 = number.as_str().parse().unwrap();
            return user_id;
        }
    }
    0
}
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn delete_character(
    ctx: Context<'_>,
    #[description = "User to delete character of"] user: User,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: i32,
) -> Result<(), Error> {
    let pool = &ctx.data().db_connection;
    let user_id = user.id.0 as i64;

    let db_character = query!(
        "SELECT df_id,character_name FROM df_characters WHERE discord_id = $1  and df_id = $2 order by created asc",
        user_id,
        character
    )
    .fetch_one(pool)
    .await?;
    let res = query!(
        "delete from df_characters where DF_ID = $1 and discord_id = $2",
        character,
        user_id
    )
    .execute(pool)
    .await?;
    dbg!(&res);
    let chars = query!(
        "SELECT df_id,character_name FROM df_characters WHERE discord_id = $1 order by created asc",
        user_id
    )
    .fetch_all(pool)
    .await?;
    let color: Color;
    let title: String;
    if res.rows_affected() == 0 {
        color = Color::DARK_RED;
        title = format!("NOT REGISTERED: {}", db_character.character_name);
        ctx.send(|f| {
            f.embed(|f| {
                f.title(title)
                    .url(format!("{}{}", CHARPAGE, character))
                    .color(color)
                    .description(format!(
                        "**DF ID:** {} [{}]({}{})",
                        db_character.df_id,
                        db_character.character_name,
                        CHARPAGE,
                        db_character.df_id
                    ))
                    .image("https://account.dragonfable.com/images/bgs/bg-df-main.jpg")
            })
        })
        .await?;
        return Ok(());
    } else {
        color = Color::DARK_GOLD;
        title = format!("Sucessfully DELETED: {}", db_character.character_name)
    }
    let chars_string = chars
        .iter()
        .map(|c| {
            format!(
                "**DF ID:** {} [{}]({}{})",
                c.df_id, c.character_name, CHARPAGE, c.df_id
            )
        })
        .chain(std::iter::once(format!(
            "~~**DF ID:** {} [{}]({}{})~~",
            db_character.df_id, db_character.character_name, CHARPAGE, db_character.df_id
        )))
        .collect::<Vec<String>>()
        .join("\n");
    ctx.send(|f| {
        f.embed(|f| {
            f.title(title)
                .url(format!("{}{}", CHARPAGE, character))
                .color(color)
                .description(chars_string)
        })
    })
    .await?;
    Ok(())
}

pub async fn autocomplete_character(
    ctx: Context<'_>,
    partial: &str,
) -> Vec<poise::AutocompleteChoice<i32>> {
    let discord_id = extract_name_from_invokation_data(&ctx.invocation_string());
    let mut ac_choices: Vec<poise::AutocompleteChoice<i32>> = Vec::new();
    let chars = if discord_id != 0 {
        let query = match partial.parse::<usize>() {
    Ok(num) => format!("SELECT df_id, character_name FROM df_characters WHERE discord_id = $1 AND CAST(df_id AS TEXT) LIKE '%{}%' ORDER BY created ASC",num),
    Err(_) => format!("SELECT df_id, character_name FROM df_characters WHERE discord_id = $1 AND character_name ILIKE '%{}%' ORDER BY created ASC",partial)
};
        let res = sqlx::query_as::<_, CharacterQuery>(&query)
            .bind(discord_id)
            .bind(partial)
            .fetch_all(&ctx.data().db_connection)
            .await;
        match res {
            Ok(chars) => chars,
            Err(_) => return ac_choices,
        }
    } else {
        let query = match partial.parse::<usize>() {
    Ok(num) => format!("SELECT df_id, character_name FROM df_characters WHERE CAST(df_id AS TEXT) LIKE '%{}%' ORDER BY created ASC",num),
    Err(_) => format!("SELECT df_id, character_name FROM df_characters WHERE character_name ILIKE '%{}%' ORDER BY created ASC",partial)
};
        sqlx::query_as::<_, CharacterQuery>(&query)
            .fetch_all(&ctx.data().db_connection)
            .await
            .unwrap()
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

#[derive(sqlx::FromRow, Debug)]
struct CharacterQuery {
    character_name: String,
    df_id: i32,
}
