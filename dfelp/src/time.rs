use chrono::{Timelike, Local,Utc};
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
