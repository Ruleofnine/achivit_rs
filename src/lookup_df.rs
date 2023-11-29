use crate::error::BotError;
use crate::manage_users::autocomplete_character;
use crate::serenity::Color;
use crate::{Context, Error};
use color_eyre::Result;
use dfelp::lookup::get_df_character;
use dfelp::{CHARPAGE, DA_IMGUR, NDA_IMGUR};
use poise::serenity_prelude::User;
use sqlx::query;
fn get_embed_color(has_da: &bool) -> (Color, String) {
    match has_da {
        true => (Color::from_rgb(254, 216, 55), DA_IMGUR.to_owned()),
        false => (Color::from_rgb(111, 101, 87), NDA_IMGUR.to_owned()),
    }
}
async fn send_character_embed(df_id: i32, ctx: Context<'_>) -> Result<()> {
    dbg!(&df_id);
    let character = get_df_character(df_id).await?;
    dbg!("2");
    match character {
        Some(character) => {
            dbg!("3");
            let (embed_color, thumbnail) = get_embed_color(&character.dragon_amulet);
            let description = character.get_discord_embed_description(df_id);
            poise::send_reply(ctx, |f| {
                f.embed(|f| {
                    f.title(format!("{}", character.name))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(embed_color)
                        .description(description)
                        .thumbnail(thumbnail)
                })
            })
            .await?;
        }
        None => {
            dbg!("4");
            poise::send_reply(ctx, |f| {
                f.embed(|f| {
                    f.title(format!("No Character With DF ID: [{}]", df_id))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description("the game character you are searching for does not exist.")
                        .image("https://account.dragonfable.com/images/bgs/bg-df-main.jpg")
                })
            })
            .await?;
        }
    }
    Ok(())
}
#[poise::command(slash_command)]
pub async fn lookup_df_id(
    ctx: Context<'_>,
    #[description = "df_id to search"] df_id: i32,
) -> Result<(), Error> {
    send_character_embed(df_id, ctx).await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn lookup_df_character(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: User,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: i32,
) -> Result<(), Error> {
    // drop just to get rid of unused
    drop(user);
    send_character_embed(character, ctx).await?;
    Ok(())
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
    let chars = query!(
        "SELECT df_id,character_name FROM df_characters WHERE discord_id = $1 order by created asc",
        user_id
    )
    .fetch_all(pool)
    .await?;
    let db_character = chars.iter().find(|c| c.df_id == character);
    let db_character = match db_character {
        Some(char) => char,
        None => {
            poise::send_reply(ctx, |f| {
                f.embed(|f| f.title(format!("Character {} Not Found", character)))
            })
            .await?;
            return Ok(());
        }
    };
    let res = query!(
        "delete from df_characters where DF_ID = $1 and discord_id = $2",
        character,
        user_id
    )
    .execute(pool)
    .await?;
    let chars = query!(
        "SELECT df_id,character_name FROM df_characters WHERE discord_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;
    let color: Color;
    let title: String;
    if res.rows_affected() == 0 {
        color = Color::DARK_RED;
        title = format!("NOT REGISTERED: {}", db_character.character_name);
        poise::send_reply(ctx, |f| {
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
    poise::send_reply(ctx, |f| {
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
