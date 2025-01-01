use crate::parsing::DFCharacterData;
use crate::{lookup_df::LookupState, parsing::Items};
use color_eyre::Result;
use rust_xlsxwriter::*;
fn write_items_to_sheet(
    worksheet: &mut Worksheet,
    list: &Items,
    row: u16,
    name: &str,
) -> Result<()> {
    let mut spacing = 0;
    worksheet.write_string(0, row, name)?;
    worksheet.write_string(2, row, format!("Unique Amount: {}", list.count()))?;
    for (i, list) in list.split_list().enumerate() {
        let list_len = list.len();
        worksheet.write_string(3 + spacing, row, Items::text(i))?;
        worksheet.write_column(5 + spacing, row, list)?;
        spacing += 4 + list_len as u32;
    }
    Ok(())
}
fn extract_data(lookup_state: LookupState) -> Option<DFCharacterData> {
    match lookup_state {
        LookupState::CharacterPage(data) => Some(data),
        _ => None,
    }
}
pub struct SheetData {
    pub user_one_name: String,
    pub user_two_name: String,
    pub user_one_unique_dif: u16,
    pub user_two_unique_dif: u16,
    pub buf: Vec<u8>,
}

pub async fn compare_sheet(
    main_state: LookupState,
    second_state: LookupState,
) -> Result<Option<SheetData>> {
    let mut workbook = Workbook::new();

    let mut char1 = extract_data(main_state);
    let mut char2 = extract_data(second_state);

    let (mut char1, mut char2) = match (char1.take(), char2.take()) {
        (Some(c1), Some(c2)) => (c1, c2),
        _ => return Ok(None),
    };

    let properties = DocProperties::new()
        .set_title(format!("{} vs {}", char1.name, char2.name))
        .set_subject(format!("Comparing {} and {}", char1.name, char2.name))
        .set_author("Achivit")
        .set_manager("Ruleofnine")
        .set_company("Rust Solutions Inc")
        .set_category("Dragonfable compare characters")
        .set_keywords("DF,Compare,Uniques")
        .set_comment("Created with Rust and rust_xlsxwriter");

    workbook.set_properties(&properties);

    let worksheet = workbook.add_worksheet();

    if let (Some(mut list1), Some(mut list2)) = (char1.item_list.take(), char2.item_list.take()) {
        let list1_keys: Vec<_> = list1.items().keys().cloned().collect();
        let list2_keys: Vec<_> = list2.items().keys().cloned().collect();

        // Retain only unique items in list1
        list1.items_mut().retain(|k, _| !list2_keys.contains(k));

        // Retain only unique items in list2
        list2.items_mut().retain(|k, _| !list1_keys.contains(k));
        write_items_to_sheet(worksheet, &list1, 0, &char1.name)?;
        write_items_to_sheet(worksheet, &list2, 5, &char2.name)?;

        let buf = workbook.save_to_buffer()?;
        let sheet_data = SheetData {
            user_one_name: char1.name.to_owned(),
            user_two_name: char2.name.to_owned(),
            user_one_unique_dif: list1.count(),
            user_two_unique_dif: list2.count(),
            buf,
        };

        return Ok(Some(sheet_data));
    }

    Ok(None)
}
