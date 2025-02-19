use crate::db::ASCEND_GUILD_ID;
use crate::guild_settings::GuildSettings;
use crate::paginate::{paginate, PaginateEmbed};
use crate::parsing::{get_discord_embed_description_flash, DFCharacterData, WarList};
use crate::requests::{
    fetch_page_with_user_agent, ASCEND_DA_IMGUR, CHARPAGE, DA_IMGUR, DF_LINK, NDA_IMGUR,
    ROLE_DA_IMGUR, USER_AGENT,
};
use crate::requirements::{check_requirements, RequirementList, RequirementListType};
use crate::rng::random_rgb;
use crate::sheets::SheetData;
use crate::update_checker::{DesignNote, UpdateCheckerFeatureFlag};
use crate::{
    serenity::{ChannelId, Color},
    Context,
};
use color_eyre::{Report, Result};
use poise::serenity_prelude::AttachmentType;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::{collections::HashMap, env, fmt::Write};
pub async fn guild_only(ctx: Context<'_>) -> Result<bool> {
    if ctx.guild().is_none() {
        guild_only_embed(ctx).await?;
        return Ok(true);
    }
    Ok(false)
}

pub async fn no_character_embed(ctx: Context<'_>) -> Result<()> {
    ctx.send( |f| {
                    f.embed(|f| {
                        f.title("No Characters Registered")
                            .color(Color::DARK_RED)
                            .description("You have no DF Characters registered, ask an administrator to regsiter your df id(s)")
                    })
                })
                .await?;
    Ok(())
}
pub async fn guild_only_with_id(ctx: Context<'_>) -> Result<Option<i64>> {
    if let Some(guild) = ctx.guild() {
        Ok(Some(guild.id.0 as i64))
    } else {
        guild_only_embed(ctx).await?;
        Ok(None)
    }
}
pub async fn send_update_embed(
    guilds: Arc<Vec<GuildSettings>>,
    dn: DesignNote,
    flag: UpdateCheckerFeatureFlag,
) -> Result<()> {
    let url = format!("{DF_LINK}{}", dn.link());
    let token = env::var("BOT_TOKEN").unwrap();
    let http = crate::serenity::Http::new(&token);
    let document = fetch_page_with_user_agent(USER_AGENT, &url).await?;
    let html = Html::parse_document(&document);
    let text_selector = Selector::parse(r#"div[class=""]"#).unwrap();
    let text = html
        .select(&text_selector)
        .next()
        .map(|element| element.text().collect::<String>())
        .unwrap_or_default();

    let description = if text.len() >= 4096 {
        let mut truncated_text = text.chars().take(4093).collect::<String>();
        truncated_text += "...";
        truncated_text
    } else {
        text
    };
    tokio::spawn(async move {
        for guild in guilds.iter() {
            let channel = ChannelId(guild.announcement_channel_id().unwrap() as u64);
            let ping = match flag {
                UpdateCheckerFeatureFlag::ForceNoPing => "".to_owned(),
                _ => format!(" <@&{}>", guild.announcement_role_id().unwrap()),
            };
            if let Err(why) = channel
                .send_message(&http, |f| {
                    f.content(format!("Release is  **LIVE!**{}", ping))
                        .embed(|e| {
                            e.title(dn.update_name())
                                .url(url.clone())
                                .color(Color::from_rgb(254, 216, 55))
                                .thumbnail(DA_IMGUR)
                                .image(dn.image())
                                .description(description.clone())
                                .author(|a| a.name(dn.poster_name()).icon_url(dn.poster_image()))
                        })
                })
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    });

    Ok(())
}
pub async fn roles_embed(
    ctx: Context<'_>,
    roles: &mut RequirementList,
    title: String,
) -> Result<()> {
    roles.sort_alphabetical();
    let description = roles
        .requirements()
        .iter()
        .fold(String::new(), |mut acc, r| {
            writeln!(
                acc,
                "**{}**\n{}",
                r.name(),
                r.description.as_ref().unwrap_or(&"".to_string())
            )
            .expect("failed to parse");
            acc
        });
    ctx.send(|f| {
        f.embed(|f| {
            f.title(title)
                .color(Color::from_rgb(1, 214, 103))
                .thumbnail(ROLE_DA_IMGUR)
                .description(description)
        })
    })
    .await?;
    Ok(())
}
pub async fn send_roles_embed(
    df_id: i32,
    char: DFCharacterData,
    ctx: Context<'_>,
    role_list_type: RequirementListType,
) -> Result<()> {
    let guild_id = match role_list_type {
        RequirementListType::Ascend => ASCEND_GUILD_ID,
        RequirementListType::Roles => ctx.guild_id().unwrap().0 as i64,
    };
    let pool = &ctx.data().db_connection;
    let name = char.name().to_owned();
    let (thumbnail, color, title) = match role_list_type {
        RequirementListType::Roles => (
            ROLE_DA_IMGUR,
            Color::from_rgb(1, 162, 197),
            format!("{}'s Eligible Roles", name),
        ),
        RequirementListType::Ascend => (
            ASCEND_DA_IMGUR,
            Color::from_rgb(0, 214, 11),
            format!("{}'s Acendancies", name),
        ),
    };
    let roles = check_requirements(&char, guild_id, pool).await?;
    let mut description = String::new();
    for role in roles.requirements() {
        description += format!("__**{}**__\n{}\n", role.name(), role.description()).as_str()
    }
    ctx.send(|f| {
        f.embed(|f| {
            f.title(title)
                .url(format!("{CHARPAGE}{df_id}"))
                .color(color)
                .thumbnail(thumbnail)
                .description(description)
        })
    })
    .await?;
    Ok(())
}
pub async fn wrong_file_type(ctx: Context<'_>, file_type: &str) -> Result<()> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("Wrong File Type! {file_type}"))
                .color(Color::DARK_RED)
                .description("The File Type must be [application/json; charset=utf-8]")
        })
    })
    .await?;
    Ok(())
}
pub async fn guild_only_embed(ctx: Context<'_>) -> Result<()> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Guild only feature!")
                .color(Color::DARK_RED)
                .description("The feature you are trying to use is only usable in a guild!")
        })
    })
    .await?;
    Ok(())
}
pub async fn no_settings_embed(ctx: Context<'_>) -> Result<()> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("There are no Guild Settings for this guild!")
                .color(Color::DARK_RED)
                .description("Guild settings still need to be created for this guild!")
        })
    })
    .await?;
    Ok(())
}
pub async fn no_roles_embed(ctx: Context<'_>) -> Result<()> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("There are no roles for this guild!")
                .color(Color::DARK_RED)
                .description("Roles still need to be created for this guild!")
        })
    })
    .await?;
    Ok(())
}
pub async fn role_init_error(ctx: Context<'_>, role_error: Report) -> Result<()> {
    ctx.send(|f| {
        f.embed(|f| {
            f.title("Error Parsing Roles!")
                .color(Color::DARK_RED)
                .description(format!(
                    "there was an error parsing roles! **Error:**\n{role_error}"
                ))
        })
    })
    .await?;
    Ok(())
}
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
pub async fn compare_not_found_embed(ctx: Context<'_>, notfound: Vec<i32>) -> Result<()> {
    let (amount, description) = match notfound.len() {
        2 => (("Both"), "These characters were"),
        _ => (("One"), "This character was"),
    };
    let chars_description = notfound.iter().fold(String::new(), |mut acc, f| {
        let _ = writeln!(acc, "[{f}]({CHARPAGE})");
        acc
    });
    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!(
                "{} of the characters you searched does not exsit",
                amount
            ))
            .color(Color::DARK_RED)
            .description(format!("{description} not found:\n{chars_description}"))
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
            f.title(character.name)
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
            f.title(name)
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
            let title = format!("{}'s Inventory", name);
            let embed = PaginateEmbed::new(
                title.as_str(),
                Some("https://imgur.com/fUyFn0I.png"),
                Color::from_rgb(105, 68, 48),
                inventory,
            );
            paginate(ctx, embed).await?;
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
pub async fn send_compare_embed(sheet: SheetData, ctx: Context<'_>) -> Result<()> {
    let title = format!("{} vs {}", sheet.user_one_name, sheet.user_two_name);
    let sheet_attachment = AttachmentType::Bytes {
        data: std::borrow::Cow::Borrowed(&sheet.buf),
        filename: format!("{}.xlsx", title.clone()),
    };
    let description1 = format!(
        "**{}** has *{}* unique items **{}** does not",
        sheet.user_one_name, sheet.user_one_unique_dif, sheet.user_two_name
    );
    let description2 = format!(
        "**{}** has *{}* unique items **{}** does not",
        sheet.user_two_name, sheet.user_two_unique_dif, sheet.user_one_name
    );

    ctx.send(|f| {
        f.embed(|f| {
            f.title(title)
                .color(random_rgb())
                .description(format!("{description1}\n{description2}"))
        })
        .attachment(sheet_attachment)
    })
    .await?;
    Ok(())
}
