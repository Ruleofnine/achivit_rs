use crate::parsing::{
    get_embed_str_partial_from_hashmap, parse_aqc_charpage, parse_aqw_charpage, parse_mech_quest_charpage, Bold
};
use crate::requests::{FLASH_USER_AGENT, USER_AGENT};
use crate::{requests::fetch_page_with_user_agent, Context, Error};
use scraper::Html;
use serenity::utils::Color;
/// Lookup a mechquest ID
#[poise::command(slash_command)]
pub async fn lookup_mechquest_id(
    ctx: Context<'_>,
    #[description = "ID to lookup"] id: i32,
) -> Result<(), Error> {
    let url = format!("https://account.mechquest.com/CharPage?id={}", id);
    let json_string = fetch_page_with_user_agent(USER_AGENT, &url).await?;
    let document = Html::parse_document(&json_string);
    let data = parse_mech_quest_charpage(document)?;
    if let Some(mechadata) = data {
        let embed_str = format!("**Level:** {}\n**Credits:** {}\n**Last Played:** {}\n**Account Type:** {}\n**Mechs in Arsenal:** {}",mechadata.level(),mechadata.credits_comma(),mechadata.last_played(),mechadata.account_type(),mechadata.mech_models());
        ctx.send(|f| {
            f.embed(|f| {
                f.title(mechadata.name())
                    .url(url)
                    .thumbnail("https://account.mechquest.com/images/logos/logo-lg-MQ.png?ver=2")
                    .color(Color::from_rgb(48, 135, 188))
                    .description(embed_str)
            })
        })
        .await?;
    } else {
        ctx.send(|f| {
            f.embed(|f| {
                f.title("Character Not Found")
                    .url(url)
                    .thumbnail("https://account.mechquest.com/images/logos/logo-lg-MQ.png?ver=2")
                    .color(Color::DARK_RED)
                    .description("The game character you are searching for does not exist.")
            })
        })
        .await?;
    }
    Ok(())
}
/// Lookup a AQC ID
#[poise::command(slash_command)]
pub async fn lookup_aqc_id(
    ctx: Context<'_>,
    #[description = "ID to lookup"] id: i32,
) -> Result<(), Error> {
    let url = format!("https://aq.battleon.com/game/flash/charview?temp={}", id);
    let json_string = fetch_page_with_user_agent(FLASH_USER_AGENT, &url).await?;
    let document = Html::parse_document(&json_string);
    let aqcdata = parse_aqc_charpage(document)?;
    if let Some(data) = aqcdata {
        let embed_str = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            get_embed_str_partial_from_hashmap(&data, "sTitle", "",Bold::All,false),
            get_embed_str_partial_from_hashmap(&data, "sClass", "",Bold::All,false),
            get_embed_str_partial_from_hashmap(&data, "iLevel", "Level: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iSTR", "STR: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iDEX", "DEX: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iINT", "INT: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iEND", "END: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iCHA", "CHA: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iLUK", "LUK: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "sGuild", "Clan: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "sSubRace", "Subrace: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "Gold", "Gold:** ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iTokens", "Z-Tokens: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "iBoxes", "Gold Gift Boxes: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "sBorn", "",Bold::All,false),
            get_embed_str_partial_from_hashmap(&data, "sBecame", "",Bold::All,false),
            get_embed_str_partial_from_hashmap(&data, "sLastPlayed", "",Bold::All,false),

        );
        ctx.send(|f| {
            f.embed(|f| {
                f.title(data.get("sName").unwrap_or(&"No Name".to_string()))
                    .url(url)
                    .thumbnail("https://www.battleon.com/images/AQ/logo-lg-aq.png")
                    .color(Color::from_rgb(248, 191, 1))
                    .description(embed_str)
            })
        })
        .await?;
    } else {
        ctx.send(|f| {
            f.embed(|f| {
                f.title("Character ID Not Found")
                    .url(url)
                    .image("https://aq.battleon.com/game/Content/images/headerbanner.gif")
                    .color(Color::DARK_RED)
                    .description("The game character you are searching for does not exist.")
            })
        })
        .await?;
    }
    Ok(())
}
/// Lookup an AQW Character
#[poise::command(slash_command)]
pub async fn lookup_aqw_character(
    ctx: Context<'_>,
    #[description = "Character Name to lookup"] name: String,
) -> Result<(), Error> {
    let url = format!("https://account.aq.com/CharPage?id={name}");
    let json_string = fetch_page_with_user_agent(FLASH_USER_AGENT , &url).await?;
    let document = Html::parse_document(&json_string);
    let data = parse_aqw_charpage(document)?;
    if let Some(data) = data {
        let embed_str = format!("{}{}{}{}{}{}{}{}{}{}{}",
            get_embed_str_partial_from_hashmap(&data, "strName", "",Bold::Suffix,false),
            get_embed_str_partial_from_hashmap(&data, "Level", "Level: ",Bold::Prefix,true),
            get_embed_str_partial_from_hashmap(&data, "strClass", "Class: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strWeaponName", "Weapon: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strArmorName", "Armor: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strHelmName", "Helm: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strCapeName", "Cape: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strPetName", "Pet: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strMiscName", "Misc: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "strFaction", "Faction: ",Bold::Prefix,false),
            get_embed_str_partial_from_hashmap(&data, "guild", "Guild: ",Bold::Prefix,false),
        );
        ctx.send(|f| {
            f.embed(|f| {
                f.title(data.get("strName").unwrap())
                    .url(url)
                    .thumbnail("https://account.aq.com/images/logos/logo-lg-AQW.png?ver=2")
                    .color(Color::from_rgb(248, 191, 1))
                    .description(embed_str)
            })
        })
        .await?;
    } else {
        ctx.send(|f| {
            f.embed(|f| {
                f.title("Not Found!")
                    .url(url)
                    .thumbnail("https://account.aq.com/images/aqw/gfx-char-boom.png")
                    .color(Color::DARK_RED)
                    .description("The game character you are searching for does not exist.")
            })
        })
        .await?;
    }
    Ok(())
}
