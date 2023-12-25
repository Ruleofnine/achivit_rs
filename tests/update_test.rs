extern crate achivit_rs;
#[cfg(test)]
mod tests {
    use achivit_rs::{update_checker::DesignNote, requests::open_file};
    use chrono::NaiveDate;
    use color_eyre::Result;
    #[tokio::test]
    async fn character_lookup() -> Result<()> {
        let str = open_file("htmls/dn12-22.html")?;
        let dn_date = NaiveDate::parse_from_str("2023-12-22", "%Y-%m-%d")?;
        let dn = DesignNote::parse_from_str(&str)?;
        assert_eq!(dn.date(),&dn_date);
        Ok(())
    }
}
