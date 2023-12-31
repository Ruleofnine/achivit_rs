use crate::lookup_df::LookupCategory;
use crate::manage_users::autocomplete_character;
use crate::paginate::{get_requirement_pages, paginate, PaginateEmbed};
use crate::parsing::{CharacterFetcher, ParsingCategory};
use crate::requirements::get_requirements;
use crate::db::INN_GUILD_ID;
use crate::rng::random_rgb;
use crate::serenity::{Color, User};
use crate::{Context, Error};
use color_eyre::Result;
//TODO
// add in varibale lookup for differnt category roles/ascends
/// Check requirements for roles/ascendancies
#[poise::command(slash_command)]
pub async fn inn_items(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
) -> Result<(), Error> {
    drop(user);
    let pool = &ctx.data().db_connection;
    let inn_list = get_requirements(INN_GUILD_ID,&pool).await?;
    let items = if let Some(df_id) = character {
        let items = CharacterFetcher::new(df_id, LookupCategory::Ascendancies)
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?
            .item_list
            .take()
            .unwrap();
        Some(items)
    } else {
        None
    };
    let pages = get_requirement_pages(inn_list, items);
    let (r, g, b) = random_rgb();
    let embed = PaginateEmbed::new("Inn Items", None, Color::from_rgb(r, g, b), pages)
        .set_empty_string("No Inn Items to display");
    paginate(ctx, embed).await?;
    Ok(())
}
