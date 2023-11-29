
use crate::Error;
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("Poise error: {0}")]
    Poise(Error),
}
