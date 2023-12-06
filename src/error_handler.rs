use crate::{Data, Error};
use log::error;
pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            if error.to_string() == "429 Too Many Requests" {
                let _ = crate::embeds::to_many_request_embed(ctx).await;
            } else {
                error!("Error in command `{}`: {:?}", ctx.command().name, error,);
            }
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
