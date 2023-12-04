use crate::{Context, Error};
use color_eyre::Result;
use crate::requests::get_random_event;
use crate::rng::random_rgb;
use chrono::{Datelike, Timelike, Local,Utc};
use num_format::{Locale, ToFormattedString};
pub fn ordinal_suffix(day: u32) -> &'static str {
    match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    }
}
pub fn swatch_time() -> f64 {
    let cet = Utc::now() + chrono::Duration::hours(1); // CET is UTC+1
    let total_seconds = cet.num_seconds_from_midnight();
    // 1 beat = 86.4 seconds
    total_seconds as f64 / 86.4
}
pub fn percentage_day_elapsed() -> f64 {
    (Local::now().num_seconds_from_midnight() as f64 / 86400.0 ) * 100.0
}
pub fn seconds_since_midnight() -> String {
    Local::now().num_seconds_from_midnight().to_formatted_string(&Locale::en)
}
pub fn seconds_until_midnight() -> String {
    (86400-Local::now().num_seconds_from_midnight()).to_formatted_string(&Locale::en)
}
///Ping the bot!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await.as_millis();
    let res = format!("{}ms 🏓", ping);
    let avatar_url =ctx.serenity_context().cache.current_user().face();
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
    let avatar_url =ctx.serenity_context().cache.current_user().face();
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
///Just gives you the Server Time :)
#[poise::command(slash_command)]
pub async fn server_time(ctx:Context<'_>) -> Result<(),Error>{
    ctx.defer().await?;
    let now = Local::now();
    let avatar_url =ctx.serenity_context().cache.current_user().face();
    let description = format!(
        "**{}, {}, {:02}{} {:04}** \n**{:02}:{:02}:{:02} {} EST**\n**Week:** {} **Day:** {:03}\n**Swatch Time:** @{:03.0}\n**Day Progress:** {:.2}%\n**Seconds Since Midnight:** {}\n**Seconds Until Midnight:** {}\n**Today in History:** {}",
        now.format("%A"),
        now.format("%B"),
        now.day(),
        ordinal_suffix(now.day()),
        now.year(),
        now.hour12().1,
        now.minute(),
        now.second(),
        now.format("%p"),
        now.iso_week().week(),
        now.ordinal(),
        swatch_time(),
        percentage_day_elapsed(),
        seconds_since_midnight(),
        seconds_until_midnight(),
        crate::requests::get_random_event().await
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
/// A Random Event on this day in history!
#[poise::command(slash_command)]
pub async fn random_event(ctx:Context<'_>) -> Result<(),Error>{
    ctx.defer().await?;
    let now = Local::now();
    let description = format!(
        "**{} {}**\n**On This day in History: **{}",
        now.format("%B"),
        now.day(),
        get_random_event().await
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
