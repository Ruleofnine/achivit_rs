#[cfg(test)]
mod tests {
    use achivit_rs::{requests::open_file, update_checker::DesignNote};
    use color_eyre::Result;
    use std::sync::Arc;
    use tokio::{
        sync::Mutex,
        time::{self, Duration},
    };
    #[tokio::test]
    async fn update_test() -> Result<()> {
        Ok(())

    }
}
