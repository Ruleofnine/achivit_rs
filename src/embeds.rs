use crate::parsing::{get_discord_embed_description_flash, DFCharacterData, WarList};
use crate::requests::{CHARPAGE, DA_IMGUR, NDA_IMGUR};
use crate::rng::random_rgb;
use crate::serenity::AttachmentType;
use crate::serenity::Color;
use crate::Context;
use color_eyre::Result;
use std::collections::HashMap;
pub async fn to_many_request_embed(ctx: Context<'_>) -> Result<()> {
    ctx.send( |f| {
        f.embed(|f| {
            f.title("Too Many Requests!")
                .color(Color::DARK_RED)
                .description("Too Many Requests were sent to the server please wait a moment before trying again!")
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

pub async fn not_found_embed(ctx: Context<'_>, df_id: i32) -> Result<()> {
    ctx.send(|f| {
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
pub async fn send_character_embed(
    character: DFCharacterData,
    df_id: i32,
    ctx: Context<'_>,
) -> Result<()> {
    let (embed_color, thumbnail) = get_embed_color(&character.dragon_amulet);
    let description = character.get_discord_embed_description(df_id);
    ctx.send(|f| {
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
pub async fn send_flash_character_embed(
    character: HashMap<String, String>,
    df_id: i32,
    ctx: Context<'_>,
) -> Result<()> {
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
    ctx.send(|f| {
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

pub async fn send_wars_embed(
    wars: WarList,
    df_id: i32,
    name: String,
    ctx: Context<'_>,
) -> Result<()> {
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
            ctx.send(|f| {
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
            ctx.send(|f| {
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
pub async fn send_inventory_embed(
    inventory: Vec<String>,
    df_id: i32,
    name: String,
    ctx: Context<'_>,
) -> Result<()> {
    match inventory.is_empty() {
        false => {
            let file = &crate::sheets::sheet().await?;
            let mut description = String::new();
            for item in inventory {
                if (item.len() + description.len()) > 4096 {
                    break;
                }
                description += &format!("{}\n", &item);
            }
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("{}'s Inventory", name))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::from_rgb(105, 68, 48))
                        .description(description)
                        .thumbnail("https://imgur.com/fUyFn0I.png")
                })
                .attachment(AttachmentType::Bytes {
                    data: std::borrow::Cow::Borrowed(file),
                    filename: "test.xlsx".to_owned(),
                })
            })
            .await?;
        }
        true => {
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("{} has no items in their Inventory", name))
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
pub async fn send_duplicates_embed(
    dups: HashMap<String, i32>,
    df_id: i32,
    name: String,
    ctx: Context<'_>,
) -> Result<()> {
    match dups.is_empty() {
        false => {
            let mut description = String::new();
            for (ele, amount) in dups {
                if (ele.len() + description.len()) > 4096 {
                    break;
                }
                description += &format!("{} (x{})\n", &ele, amount);
            }
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("{}'s Duplicates", name))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(random_rgb())
                        .description(description)
                        .thumbnail("https://imgur.com/fUyFn0I.png")
                })
            })
            .await?;
        }
        true => {
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("{} has no Duplicates", name))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description("the character you are searching for has no Duplicate Items.")
                        .thumbnail("https://imgur.com/fUyFn0I.png")
                })
            })
            .await?;
        }
    };
    Ok(())
}
pub async fn wrong_cache_embed(df_id: i32, ctx: Context<'_>, flash: bool) -> Result<()> {
    let cached = match flash {
        true => "Flash Charatcer Page",
        false => "Non-Flash Character Page",
    };
    ctx.send( |f| {
                f.embed(|f| {
                    f.title(format!("Character page cached..."))
                        .url(format!("{}{}", CHARPAGE, df_id))
                        .color(Color::DARK_RED)
                        .description(format!("This character page is currently cached as  a **{}** so it cannot be used to register yet. Wait a moment and try again.",cached))
                        .image("https://account.dragonfable.com/images/bgs/bg-df-main.jpg")
                })
            })
            .await?;
    Ok(())
}

