use rand::{Rng,seq::SliceRandom};

use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub date: String,
    pub url: String,
    pub data: EventData,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct EventData {
    Events: Vec<Event>,
}
#[derive(Debug, Deserialize)]
pub struct Event {
    pub year: String,
    pub text: Option<String>,
    pub html: String,
    pub no_year_html: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub title: String,
    pub link: String,
}
pub fn random_event(json: Value) -> Option<Event> {
    let mut data: Data = serde_json::from_value(json).expect("Failed to parse JSON");
    if !data.data.Events.is_empty() {
        let idx = rand::thread_rng().gen_range(0..data.data.Events.len());
        Some(data.data.Events.remove(idx))
    } else {
        None
    }
}
pub fn event_parsing(event: Option<Event>) -> String {
    match event {
        Some(event) => crate::parsing::convert_html_to_discord_format(&event.html),
        None => "None".to_string(),
    }
}

pub fn random_rgb() -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    (r, g, b)
}
