#[cfg(any(feature="reqwest-tests",rust_analyzer))]
mod tests {
    use color_eyre::Result;
    use achivit_rs::requests::{fetch_page_with_user_agent,DESIGN_NOTES_LINK,USER_AGENT};
    use achivit_rs::update_checker::DesignNote;
    #[tokio::test]
    async fn update_test() -> Result<()> {
    let last_dn_str = fetch_page_with_user_agent(USER_AGENT, DESIGN_NOTES_LINK).await?;
    let last_dn = DesignNote::parse_from_str(&last_dn_str)?;
    let two_days_ago = chrono::NaiveDate::from_ymd_opt(2024, 12, 21).unwrap();
    assert!(last_dn.date()>&two_days_ago);
        Ok(())

    }
}
