use crate::{Context, Error,dev_tools};
use color_eyre::Result;
use dfelp::rng::random_rgb;
use chrono::{Datelike, Timelike, Local};
///Ping the bot!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await.as_millis();
    let res = format!("{}ms 🏓", ping);
    let avatar_url = dev_tools::get_bot_avatar(&ctx).await?;
    poise::send_reply(ctx, |f| {
        f.embed(|f| f.color(random_rgb()).title("Pong!").description(res).thumbnail(avatar_url))
    })
    .await?;
    Ok(())
}
/// Time since the bot started
#[poise::command(slash_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    fn divmod(numerator: u64, denominator: u64) -> (u64, u64) {
        (numerator / denominator, numerator % denominator)
    }
    let uptime_duration = ctx.data().start_time.elapsed();
    let (hours, remainder) = divmod(uptime_duration.as_secs(), 3600);
    let (minutes, seconds) = divmod(remainder, 60);
    let description = format!(
        "{} hours, {} minutes, and {} seconds",
        hours, minutes, seconds
    );
    let avatar_url = dev_tools::get_bot_avatar(&ctx).await?;
    poise::send_reply(ctx, |f| {
        f.embed(|f| {
            f.color(random_rgb())
                .title("Uptime!")
                .description(description)
                .thumbnail(avatar_url)
        })
    })
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn server_time(ctx:Context<'_>) -> Result<(),Error>{
    ctx.defer().await?;
    let now = Local::now();
    let avatar_url = dev_tools::get_bot_avatar(&ctx).await?;
    let description = format!(
        "**{}, {}, {:02}{} {:04}** \n**{:02}:{:02}:{:02} {} EST**\n**Week:** {} **Day:** {:03}\n**Swatch Time:** @{:03.0}\n**Day Progress:** {:.2}%\n**Seconds Since Midnight:** {}\n**Seconds Until Midnight:** {}\n**Today in History:** {}",
        now.format("%A"),
        now.format("%B"),
        now.day(),
        dfelp::time::ordinal_suffix(now.day()),
        now.year(),
        now.hour12().1,
        now.minute(),
        now.second(),
        now.format("%p"),
        now.iso_week().week(),
        now.ordinal(),
        dfelp::time::swatch_time(),
        dfelp::time::percentage_day_elapsed(),
        dfelp::time::seconds_since_midnight(),
        dfelp::time::seconds_until_midnight(),
        dfelp::requests::get_random_event().await
    );
    poise::send_reply(ctx, |f| {
        f.embed(|f| {
            f.color(random_rgb())
                .title("Server time")
                .description(description)
                .thumbnail(avatar_url)
        })
    })
    .await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn random_event(ctx:Context<'_>) -> Result<(),Error>{
    ctx.defer().await?;
    let now = Local::now();
    let description = format!(
        "**{:02}**\n**On This day in History: **{}",
        now.day(),
        dfelp::requests::get_random_event().await
    );
    poise::send_reply(ctx, |f| {
        f.embed(|f| {
            f.color(random_rgb())
                .title("Random Event")
                .description(description)
        })
    })
    .await?;
    Ok(())
}
