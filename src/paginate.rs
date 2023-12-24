use crate::parsing::Items;
use crate::requirements::RequirementList;
use crate::serenity::{CollectComponentInteraction, InteractionResponseType};
use crate::Context;
use color_eyre::Result;
use getset::Getters;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateButton;
use serenity::utils::Color;
const MAX_PAGE_LENGTH: usize = 1365;
#[derive(Getters)]
#[getset(get = "pub")]
pub struct PaginateEmbed<'a> {
    pages: Vec<String>,
    title: &'a str,
    thumbnail: Option<&'a str>,
    color: Color,
    footer: String,
    current_page: usize,
    empty_string: Option<&'a str>,
}
impl<'a> PaginateEmbed<'a> {
    fn get_current_page(&self) -> &str {
        &self.pages()[self.current_page]
    }
    pub fn new(
        title: &'a str,
        thumbnail: Option<&'a str>,
        color: Color,
        pages: Vec<String>,
    ) -> PaginateEmbed<'a> {
        PaginateEmbed {
            title,
            thumbnail,
            color,
            footer: format!("Page {} of {}", 1, pages.len()),
            pages,
            current_page: 0,
            empty_string: None,
        }
    }
    fn next_clicked(&mut self) {
        self.next_page();
        if self.current_page >= self.pages().len() {
            self.reset_page()
        }
        self.update_footer()
    }
    fn next_page(&mut self) {
        self.current_page += 1
    }
    fn previous_clicked(&mut self) {
        self.current_page = self
            .current_page
            .checked_sub(1)
            .unwrap_or(self.pages().len() - 1);
        self.update_footer()
    }
    fn update_footer(&mut self) {
        self.footer = format!("Page {} of {}", self.current_page() + 1, self.pages().len())
    }
    fn reset_page(&mut self) {
        self.current_page = 0;
    }
    pub fn set_empty_string(mut self, string: &'a str) -> PaginateEmbed {
        self.empty_string = Some(string);
        self
    }
    fn check_empty(&mut self) {
        if self.pages().is_empty() {
            let string = match self.empty_string() {
                Some(str) => str,
                None => "No Data to paginate",
            };
            self.pages = vec![String::from(string)]
        };
        self.update_footer()
    }
}

fn check_new_page(
    pages: &mut Vec<String>,
    page_index: &mut usize,
    current_len: &mut usize,
    next_len: usize,
) {
    if *current_len + next_len >= MAX_PAGE_LENGTH {
        *page_index += 1;
        pages.push(String::with_capacity(4096));
        *current_len = 0;
    }
}
pub fn paginate_item(
    pages: &mut Vec<String>,
    item: String,
    current_len: &mut usize,
    current_page: &mut usize,
) {
    check_new_page(pages, current_page, current_len, item.len());
    *current_len += item.len();
    pages[*current_page].push_str(&item);
}

pub fn get_requirement_pages(req_list: RequirementList, items: Option<Items>) -> Vec<String> {
    let mut pages: Vec<String> = vec![String::new()];
    let mut current_page = 0;
    let mut current_len = 0;
    let items_present = items.is_some();
    for challenge in req_list.requirements() {
        let challenge_text = format!("__**{}**__\n", challenge.name());
        check_new_page(
            &mut pages,
            &mut current_page,
            &mut current_len,
            challenge_text.len(),
        );
        pages[current_page].push_str(&challenge_text);
        current_len += challenge_text.len();
        for item in challenge.required() {
            if items_present
                && dbg!(items
                    .as_ref()
                    .map_or(false, |items| !items.items().contains_key(item)))
                || items.is_none()
            {
                let item_text = format!("{}\n", item);
                check_new_page(
                    &mut pages,
                    &mut current_page,
                    &mut current_len,
                    item_text.len(),
                );
                pages[current_page].push_str(&item_text);
                current_len += item_text.len();
            };
        }
    }
    // Remove the last page if it is empty
    if pages.last().map_or(false, |p| p.is_empty()) {
        pages.pop();
    }
    pages
}
pub async fn paginate<'a>(
    ctx: Context<'_>,
    mut paginate_struct: PaginateEmbed<'a>,
) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    paginate_struct.check_empty();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);
    let mut left_button = CreateButton::default();
    left_button.label("◀").custom_id(&prev_button_id);
    let mut right_button = CreateButton::default();
    right_button.label("▶").custom_id(&next_button_id);
    let mut action_row = CreateActionRow::default();
    action_row
        .add_button(left_button)
        .add_button(right_button)
        .build();
    ctx.send(|f| {
        f.components(|f| {
            if paginate_struct.pages().len() > 1 {
                f.add_action_row(action_row)
            } else {
                f
            }
        })
        .embed(|f| {
            if paginate_struct.thumbnail().is_some() {
                f.thumbnail(&paginate_struct.thumbnail().as_ref().unwrap());
            }
            f.title(paginate_struct.title())
                .description(paginate_struct.get_current_page())
                .color(*paginate_struct.color())
                .footer(|f| f.text(paginate_struct.footer()))
            // f.thumbnail(paginate_struct.thumbnail().unwrap())
        })
    })
    .await?;

    while let Some(press) = CollectComponentInteraction::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            paginate_struct.next_clicked()
        } else if press.data.custom_id == prev_button_id {
            paginate_struct.previous_clicked()
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_interaction_response(ctx, |b| {
                b.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| {
                        b.embed(|b| {
                            if paginate_struct.thumbnail().is_some() {
                                b.thumbnail(paginate_struct.thumbnail().unwrap());
                            }
                            b.title(paginate_struct.title())
                                .description(paginate_struct.get_current_page())
                                .color(*paginate_struct.color())
                                .footer(|f| f.text(paginate_struct.footer()))
                        })
                    })
            })
            .await?;
    }
    Ok(())
}
#[cfg(test)]
mod tests{
    use crate::parsing::{FileFetcher, ParsingCategory};
    use super::*;
    #[tokio::test]
    async fn paginate_inventory_test() -> Result<()> {
        let (_, pages) = FileFetcher::new("htmls/3ach.html")
            .category(ParsingCategory::Inventory)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_inventory_data()?;
        pages
            .iter()
            .for_each(|page| assert!(page.len() < MAX_PAGE_LENGTH));
        Ok(())
    }

}
