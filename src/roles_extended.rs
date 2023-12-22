use crate::manage_users::autocomplete_character;
use crate::paginate::{paginate_requirements,get_requirement_pages};
use crate::requirements::get_requirements;
use crate::serenity::User;
use crate::{Context, Error};
use color_eyre::Result;
/// Check requirements for roles/ascendancies
#[poise::command(slash_command)]
pub async fn inn_items(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
) -> Result<(), Error> {
    let mut inn_items = get_requirements("InnList.json")?;
    inn_items.sort_alphabetical();
    let pages = get_requirement_pages(inn_items)?;
    paginate_requirements(ctx, pages).await?;
    Ok(())
}
