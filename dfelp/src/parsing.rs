use crate::CHARPAGE;
use chrono::NaiveDate;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use soup::prelude::*;
use std::fs;
pub fn convert_html_to_discord_format(input: &str) -> String {
    let re = Regex::new(r#"<a href="(?P<url>[^"]+)"[^>]*>(?P<text>[^<]+)</a>"#).unwrap();
    re.replace_all(input, "[${text}](${url})").to_string()
}
pub fn parse_df_character_from_file(file_path: &str) -> Result<Option<DFCharacterData>> {
    let data = fs::read_to_string(file_path)?;
    let document = Soup::new(&data);
    Ok(parse_df_character(document))
}

#[derive(Debug)]
pub struct Dragon {
    name: String,
    dragon_type: String,
}

impl Dragon {
    fn new(dragon_str: String) -> Dragon {
        let mut split = dragon_str.splitn(2, '(');
        let name = split.next().map(|s| s.trim().to_string()).unwrap();
        let dragon_type = split
            .next()
            .map(|s| s.trim_end_matches(')').to_string())
            .unwrap();
        Dragon { name, dragon_type }
    }
}
#[derive(Debug)]
pub struct DFCharacterData {
    pub id: u32,
    pub name: String,
    pub dragon: Option<Dragon>,
    pub dragon_amulet: bool,
    pub dmk: Option<String>,
    pub gold: i32,
    pub level: u8,
    pub item_count: u16,
    pub unique_item_count: u16,
    pub nda_count: u16,
    pub da_count: u16,
    pub dc_count: u16,
    pub artifact_count: u16,
    pub last_played: NaiveDate,
    pub wars: WarList,
}
impl DFCharacterData {
    fn default() -> DFCharacterData {
        DFCharacterData {
            id: 0,
            name: "Default".to_string(),
            dragon: None,
            dragon_amulet: false,
            dmk: None,
            gold: 0,
            level: 1,
            item_count: 0,
            da_count: 0,
            nda_count: 0,
            dc_count: 0,
            artifact_count: 0,
            unique_item_count: 0,
            last_played: NaiveDate::default(),
            wars: WarList::new(),
        }
    }
    fn calc_item_count(&mut self) {
        self.item_count = self.da_count + self.dc_count + self.nda_count + self.artifact_count
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn get_da_str(&self) -> String {
        match self.dragon_amulet {
            true => "**Dragon Amulet Owner**\n".to_string(),
            false => "".to_string(),
        }
    }
    pub fn get_dragon_str(&self) -> String {
        match &self.dragon {
            Some(dragon) => format!("**Dragon:** {} ({})\n", dragon.name, dragon.dragon_type),
            None => "".to_string(),
        }
    }
    pub fn get_dmk_str(&self) -> String {
        match &self.dmk {
            Some(dmk) => format!("{}\n", dmk.to_string()),
            None => "".to_string(),
        }
    }
    pub fn get_gold(&self) -> String {
        match self.gold {
            0 => "".to_string(),
            _ => format!("**Gold:** {}\n", self.gold.to_formatted_string(&Locale::en)),
        }
    }
    pub fn get_level(&self) -> String {
        format!("**Level:** {}\n", self.level)
    }
    pub fn get_last_played(&self) -> String {
        self.last_played
            .format("**Last Played:** %A, %B %e, %Y\n")
            .to_string()
    }
    pub fn get_total_waves(&self) -> String {
        match self.wars.war_list.len() {
            0 => "".to_string(),
            _ => format!(
                "**War Waves:** {}\n",
                self.wars
                    .calc_waves_cleared()
                    .to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_item_count(&self) -> String {
        match self.item_count {
            0 => "".to_string(),
            _ => format!(
                "**Items:** {}\n",
                self.item_count.to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_unique_item_count(&self) -> String {
        match self.unique_item_count {
            0 => "".to_string(),
            _ => format!(
                "**Unique Items:** {}\n",
                self.unique_item_count.to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_nda_item_count(&self) -> String {
        match self.nda_count {
            0 => "".to_string(),
            _ => format!(
                "**NDA Items:** {}\n",
                self.nda_count.to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_da_item_count(&self) -> String {
        match self.da_count {
            0 => "".to_string(),
            _ => format!(
                "**DA Items:** {}\n",
                self.da_count.to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_dc_item_count(&self) -> String {
        match self.dc_count {
            0 => "".to_string(),
            _ => format!(
                "**DC Items:** {}\n",
                self.dc_count.to_formatted_string(&Locale::en)
            ),
        }
    }
    pub fn get_discord_embed_description(&self, id: i32) -> String {
        format!(
            "**DF ID: **[{}]({}{})\n{}{}{}{}{}{}{}{}{}{}{}",
            id,
            CHARPAGE,
            id,
            self.get_da_str(),
            self.get_dragon_str(),
            self.get_level(),
            self.get_gold(),
            self.get_last_played(),
            self.get_total_waves(),
            self.get_item_count(),
            self.get_unique_item_count(),
            self.get_nda_item_count(),
            self.get_da_item_count(),
            self.get_dc_item_count()
        )
    }
}

#[derive(Debug)]
pub struct War {
    pub warlabel: String,
    pub waves: String,
    pub rares: String,
}
impl War {
    pub fn new(warlabel: String, war_text: &str) -> War {
        let mut war_split = war_text.splitn(2, ',');
        let waves = war_split.next().map(|s| s.trim().to_string()).unwrap();
        let rares = war_split.next().map(|s| s.trim().to_string()).unwrap();
        War {
            warlabel,
            waves,
            rares,
        }
    }
}
#[derive(Debug)]
pub struct WarList {
    war_list: Vec<War>,
}

impl WarList {
    fn new() -> Self {
        WarList {
            war_list: Vec::new(),
        }
    }
    fn push_war(&mut self, war: War) {
        self.war_list.push(war);
    }
    pub fn calc_waves_cleared(&self) -> u16 {
        self.war_list.iter().fold(0, |c, x| {
            x.waves.replace(" waves", "").parse::<u16>().unwrap() + c
        })
    }
}
pub fn parse_df_character(document: Soup) -> Option<DFCharacterData> {
    let mut character = DFCharacterData::default();
    let charpagedetails =
        if let Some(div) = document.tag("div").attr("id", "charpagedetails").find() {
            div
        } else {
            return None;
        };

    character.name = charpagedetails
        .tag("h1")
        .find()
        .expect("No Name Found")
        .text()
        .trim()
        .to_string();

    let card_body = charpagedetails
        .class("card-body")
        .find()
        .expect("No Card card-body");
    let children: Vec<_> = card_body.children().collect();
    character.gold = children
        .iter()
        .position(|child| child.text().contains("Gold:"))
        .and_then(|label_index| children.get(label_index + 1))
        .and_then(|node| node.text().trim().parse::<i32>().ok())
        .unwrap_or(0);
    character.level = children
        .iter()
        .position(|child| child.text().contains("Level:"))
        .and_then(|label_index| children.get(label_index + 1))
        .and_then(|node| node.text().trim().parse::<u8>().ok())
        .unwrap_or(1);
    character.dragon = card_body
        .tag("span")
        .find()
        .map(|span_tag| Dragon::new(span_tag.text()))
        .or(None);
    character.dragon_amulet = children
        .iter()
        .any(|child| child.text().contains("Dragon Amulet Owner"));
    character.dmk = children
        .iter()
        .find(|child| child.text().contains("Doom"))
        .map(|node| node.text().trim().to_string());
    character.last_played = children
        .iter()
        .position(|child| child.text().contains("Last Played:"))
        .and_then(|label_index| children.get(label_index + 1))
        .map(|node| {
            NaiveDate::parse_from_str(node.text().trim(), "%A, %B %d, %Y").unwrap_or_default()
        })
        .unwrap_or_default();
    dbg!("8");
    let mut unique_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut war_name: String = "Error".to_string();
    dbg!(charpagedetails.tag("span").recursive(false).find_all().count());
    dbg!("9");
    for span in charpagedetails.recursive(false).tag("span").find_all() {
        // dbg!(&span.text().trim());
        let item_type = span.get("class");
        let item_type = match item_type {
            Some(item) => item,
            None => continue,
        };
        let item_type = span.get("class").unwrap();
        let item_name = span.text().trim().to_string();
        let mut item = true;
        match &item_type[..] {
            "gold" => {
                character.nda_count += 1;
            }
            "coins" => {
                character.dc_count += 1;
            }
            "amulet" => {
                character.da_count += 1;
            }
            "artifact" => {
                character.artifact_count += 1;
            }
            "warlabel" => {
                item = false;
                war_name = item_name.clone()
            }
            "mx-2 d-inline-block" => {
                item = false;
                character
                    .wars
                    .push_war(War::new(war_name.clone(), &item_name));
            }
            _ => {
                panic!("UnexpectedItemType");
            }
        }
        if item {
            unique_names.insert(item_name);
        }
    }
    character.unique_item_count = unique_names.len() as u16;
    character.calc_item_count();
    Some(character)
}
pub async fn parse_df_character_wars_only(document: Soup) -> Result<Option<WarList>> {
    let charpagedetails =
        if let Some(div) = document.tag("div").attr("id", "charpagedetails").find() {
            div
        } else {
            return Ok(None);
        };
    let mut war_list = WarList::new();
    let mut war_name: String = "Error".to_string();
    let card_divs = charpagedetails
        .tag("div")
        .class("card")
        .find_all()
        .into_iter()
        .filter(|e| e.text().contains("War Records"))
        .next();
    let war_card = match card_divs {
        Some(card) => card,
        None => return Ok(Some(war_list)),
    };
    for span in war_card.tag("span").find_all() {
        let war_type = span.get("class").unwrap();
        let name = span.text().trim().to_string();
        match &war_type[..] {
            "warlabel" => war_name = name.clone(),
            "mx-2 d-inline-block" => war_list.push_war(War::new(war_name.clone(), &name)),
            other => {
                let error_item = format!("UnexpectedItemType: {}", other.to_owned());
                return Err(color_eyre::eyre::eyre!(error_item));
            }
        }
    }
    Ok(Some(war_list))
}
