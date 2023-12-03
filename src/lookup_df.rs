use crate::manage_users::autocomplete_character;
use std::collections::HashMap;
use crate::serenity::Color;
use crate::{Context, Error};
use color_eyre::Result;
use dfelp::lookup::*;
use dfelp::parsing::{get_discord_embed_description_flash, LookupState, DFCharacterData, WarList};
use dfelp::{CHARPAGE, DA_IMGUR, NDA_IMGUR};
use poise::serenity_prelude::User;
use sqlx::{query, PgPool};
#[allow(non_camel_case_types)]
#[derive(poise::ChoiceParameter, PartialEq)]
enum LookupCategory {
    CharacterPage,
    FlashCharacterPage,
    Inventory,
    Wars,
    Duplicates,
}
impl LookupCategory {
    async fn lookup(&self, df_id: i32) -> Result<LookupState> {
        let state = match self {
            LookupCategory::CharacterPage => get_df_character(df_id).await?,
            LookupCategory::FlashCharacterPage =>get_df_character_flash(df_id).await?,
            LookupCategory::Inventory => get_df_character_inventory_only(df_id).await?,
            LookupCategory::Wars => get_df_character_wars_only(df_id).await?,
            LookupCategory::Duplicates => get_df_character_duplicates(df_id).await?,
        };
        Ok(state)
    }
    async fn send_embed(self,state:LookupState,ctx: Context<'_>,df_id: i32)->Result<()>{
        match state{
            LookupState::Fail(flash) => wrong_cache_embed(df_id, ctx, flash).await?,
            LookupState::NotFound => {not_found_embed(ctx, df_id).await?;},
            LookupState::CharacterPage(char) => {send_character_embed(char,df_id, ctx).await?},
            LookupState::FlashCharatcerPage(char) => {send_flash_character_embed(char,df_id, ctx).await?},
            LookupState::Wars(name,wars)=>{send_wars_embed(wars,df_id,name, ctx).await?},
            LookupState::Inventory(name,inventory)=>send_inventory_embed(inventory,df_id, name, ctx).await?,
            LookupState::Duplicates(name,dups)=>send_duplicates_embed(dups,df_id, name, ctx).await?
        };
        Ok(())
    }
}

pub async fn not_found_embed(ctx: Context<'_>, df_id: i32) -> Result<()> {
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
    Ok(())
}
fn get_embed_color(has_da: &bool) -> (Color, String) {
    match has_da {
        true => (Color::from_rgb(254, 216, 55), DA_IMGUR.to_owned()),
        false => (Color::from_rgb(111, 101, 87), NDA_IMGUR.to_owned()),
    }
}
async fn send_character_embed(character:DFCharacterData, df_id: i32, ctx: Context<'_>) -> Result<()> {
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
      Ok(())
}
async fn send_flash_character_embed(character:HashMap<String,String>,df_id: i32, ctx: Context<'_>) -> Result<()> {
            let (_, thumbnail) = get_embed_color(match character.get("DA").unwrap().as_str() {
                "0" => &false,
                _ => &true,
            });
            let name = character.get("Name").unwrap().to_owned();
            let color_value = character
                .get("BaseColor")
                .unwrap()
                .to_owned()
                .parse::<i32>()
                .unwrap_or_default();
            let embed_color = Color::from(color_value);
            let description = get_discord_embed_description_flash(character, df_id);
            poise::send_reply(ctx, |f| {
                f.embed(|f| {
                    f.title(format!("{}", name))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(embed_color)
                        .description(description)
                        .thumbnail(thumbnail)
                })
            })
            .await?;
        Ok(())
}

async fn send_wars_embed(wars:WarList,df_id: i32,name:String, ctx: Context<'_>) -> Result<()> {
        match wars.is_empty() {
            false => {
                let mut description = format!(
                    "__**Total Waves Cleared: {}**__\n",
                    wars.total_waves_string()
                );
                for ele in wars.vec_of_war_strings() {
                    if ele.len() + description.len() > 4096 {
                        break;
                    }
                    description += &ele
                }
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!("{}'s War Record", name))
                            .url(format!("{}{}", CHARPAGE, df_id))
                            .color(Color::DARK_RED)
                            .description(description)
                            .thumbnail("https://imgur.com/skAB2BR.png")
                    })
                })
                .await?;
            }
            true => {
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!("{} has No War Records", name))
                            .url(format!("{}{}", CHARPAGE, df_id))
                            .color(Color::DARK_RED)
                            .description("the character you are searching for has no war records.")
                            .thumbnail("https://imgur.com/skAB2BR.png")
                    })
                })
                .await?;
            }
        };
    Ok(())
}
async fn send_inventory_embed(inventory:Vec<String>,df_id: i32,name:String, ctx: Context<'_>) -> Result<()> {
        match inventory.is_empty() {
            false => {
                let mut description = String::new();
                for item in inventory {
                    if (item.len() + description.len()) > 4096 {
                        break;
                    }
                    description += &format!("{}\n", &item);
                }
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!("{}'s Inventory", name))
                            .url(format!("{}{}", CHARPAGE, df_id))
                            .color(Color::from_rgb(105, 68, 48))
                            .description(description)
                            .thumbnail("https://imgur.com/fUyFn0I.png")
                    })
                })
                .await?;
            }
            true => {
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!(
                            "{} has no items in their Inventory",
                            name
                        ))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description(
                            "the character you are searching for has no Items in their Inventory.",
                        )
                        .thumbnail("https://imgur.com/fUyFn0I.png")
                    })
                })
                .await?;
            }
        };
    Ok(())
}
async fn send_duplicates_embed(dups:HashMap<String,i32>,df_id: i32,name:String, ctx: Context<'_>) -> Result<()> {
        match dups.is_empty() {
            false => {
                let mut description = String::new();
                for (ele, amount) in dups {
                    if (ele.len() + description.len()) > 4096 {
                        break;
                    }
                    description += &format!("{} (x{})\n", &ele, amount);
                }
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!("{}'s Duplicates", name))
                            .url(format!("{}{}", CHARPAGE, df_id))
                            .color(dfelp::rng::random_rgb())
                            .description(description)
                            .thumbnail("https://imgur.com/fUyFn0I.png")
                    })
                })
                .await?;
            }
            true => {
                poise::send_reply(ctx, |f| {
                    f.embed(|f| {
                        f.title(format!("{} has no Duplicates", name))
                            .url(format!("{}{}", CHARPAGE, df_id))
                            .color(Color::DARK_RED)
                            .description(
                                "the character you are searching for has no Duplicate Items.",
                            )
                            .thumbnail("https://imgur.com/fUyFn0I.png")
                    })
                })
                .await?;
            }
        };
      Ok(())
}
pub async fn wrong_cache_embed(df_id: i32,ctx: Context<'_>,flash:bool)->Result<()>{
    let cached = match flash{
        true => "Flash Charatcer Page",
        false => "Non-Flash Character Page"
    };
            poise::send_reply(ctx, |f| {
                f.embed(|f| {
                    f.title(format!("Character page cached..."))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description(format!("This character page is currently cached as  a **{}** so it cannot be used to resigter yet. Wait a moment and try again.",cached))
                        .image("https://account.dragonfable.com/images/bgs/bg-df-main.jpg")
                })
            })
            .await?;
    Ok(())
}

async fn query_with_id(id: u64, pool: &PgPool) -> Option<i32> {
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
#[poise::command(slash_command)]
pub async fn lookup_df_character(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
    category: Option<LookupCategory>,
) -> Result<(), Error> {
    let category = match category {
        None => LookupCategory::CharacterPage,
        Some(cat) => cat,
    };
    let pool = &ctx.data().db_connection;
    let df_id = match (character, user) {
        (Some(character), _) => Some(character),
        (None, None) => query_with_id(ctx.author().id.0, pool).await,
        (None, Some(user)) => query_with_id(user.id.0, pool).await,
    };
    let df_id = match df_id {
        Some(df_id) => df_id,
        None => {
            poise::send_reply(ctx, |f| {
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
    let state =category.lookup(df_id).await?;
    Ok(category.send_embed(state, ctx, df_id).await?)

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
