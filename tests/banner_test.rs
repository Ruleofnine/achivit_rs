extern crate achivit_rs;
mod tests {
    use achivit_rs::print_banner;
    use anyhow::Result;
    #[tokio::test]
    async fn banner_test() -> Result<()> {
        print_banner(true);
        Ok(())
    }
}
