use crate::Context;
use crate::requirements::RequirementList;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateButton;
use poise::CreateReply;
use color_eyre::Result;
use std::fmt::Write;
    fn check_new_page(pages:&mut Vec<String>,page_length:&mut usize,page_index:&mut usize){
        if *page_length >= 4096usize{
            *page_index += 1usize;
            *page_length = 0usize;
            pages.push(String::new())
        }
    }
    pub fn get_requirement_pages(req_list:RequirementList) -> Result<Vec<String>> {
        let mut pages: Vec<String> = vec![String::new()];
        let mut current_page = 0;
        let mut page_length = 0;
        for challenge in req_list.requirements() {
            page_length += challenge.name().len()+1;
            check_new_page(&mut pages, &mut page_length, &mut current_page);
            writeln!(pages.get_mut(current_page).unwrap(), "{}", challenge.name())?;
            for item in challenge.required() {
                page_length += item.len()+1;
                println!("{current_page}:{page_length}");
                check_new_page(&mut pages, &mut page_length, &mut current_page);
                writeln!(pages.get_mut(current_page).unwrap(), "{}", item)?;
            }
        }
        Ok(pages)
    }
pub async fn paginate_requirements(ctx: Context<'_>, pages: Vec<String>) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);
    let mut left_button = CreateButton::default();
    left_button.label("◀").custom_id(prev_button_id);
    let mut right_button = CreateButton::default();
    right_button.label("▶").custom_id(next_button_id);
    let mut action_row = CreateActionRow::default();
    let mut pages_iter = pages.iter();
    let page_one = pages_iter.next().unwrap();
    dbg!(&page_one.len());
    action_row
        .add_button(left_button)
        .add_button(right_button);
    let mut reply = CreateReply::default();
    reply
        .embed(|e|e.description(pages_iter.next().unwrap())).components(|c|c.set_action_row(action_row));
    ctx.send(|f| f.embed(|f|f.title("test").description(pages_iter.next().unwrap())))
    .await?;
    Ok(())
}
//     });
//     ctx.send(|f| {
//         f.embed(|f| {
//             f.title(format!("{guild_name} Roles"))
//                 .color(Color::from_rgb(1, 214, 103))
//                 .thumbnail(ROLE_DA_IMGUR)
//                 .description(description)
//         })
//     })
//     .await?;
//     Ok(())
// }
