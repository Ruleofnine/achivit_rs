use rust_xlsxwriter::*;
use color_eyre::Result;
use std::io::Cursor;
use tokio::fs::File;
use std::io::Write;
// use tokio::io::AsyncWriteExt; 
pub async fn sheet() -> Result<Vec<u8>>{
    let mut workbook = Workbook::new();
    let properties = DocProperties::new()
        .set_title("This is an example spreadsheet")
        .set_subject("That demonstrates document properties")
        .set_author("A. Rust User")
        .set_manager("J. Alfred Prufrock")
        .set_company("Rust Solutions Inc")
        .set_category("Sample spreadsheets")
        .set_keywords("Sample, Example, Properties")
        .set_comment("Created with Rust and rust_xlsxwriter");

    workbook.set_properties(&properties);
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Hello")?;
    worksheet.write_string(0, 1, "Hello")?;
    let buf = workbook.save_to_buffer()?;
    Ok(buf)
}
