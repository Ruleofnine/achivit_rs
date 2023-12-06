use crate::lookup_df::LookupState;
use crate::requests::{CHARPAGE, COLOR_SITE};
use chrono::NaiveDate;
use color_eyre::Result;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::collections::{HashMap, HashSet};
use std::fs;
pub fn convert_html_to_discord_format(input: &str) -> String {
    let re = Regex::new(r#"<a href="(?P<url>[^"]+)"[^>]*>(?P<text>[^<]+)</a>"#).unwrap();
    re.replace_all(input, "[${text}](${url})").to_string()
}
pub fn parse_df_character_from_file(file_path: &str) -> Result<LookupState> {
    let data = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&data);
    Ok(parse_df_character(&document))
}
pub fn parse_df_character_flash_from_file(file_path: &str) -> Result<LookupState> {
    let data = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&data);
    Ok(parse_df_character_flash(&document))
}
pub fn parse_df_character_wars_from_file(file_path: &str) -> Result<LookupState> {
    let data = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&data);
    Ok(parse_df_character_wars_only(&document))
}
pub fn parse_df_character_inventory_only_from_file(file_path: &str) -> Result<LookupState> {
    let data = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&data);
    Ok(parse_df_character_inventory_only(&document))
}
pub fn parse_df_character_duplicates_from_file(file_path: &str) -> Result<LookupState> {
    let data = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&data);
    Ok(parse_df_character_duplicates(&document))
}

#[derive(Debug)]
pub struct Dragon {
    pub name: String,
    pub dragon_type: String,
}

impl Dragon {
    fn new(mut dragon_str: String) -> Dragon {
        dragon_str.truncate(dragon_str.len() - 7);
        let mut split_name: Vec<_> = dragon_str.split_inclusive("(").collect();
        let dragon_type = split_name
            .pop()
            .expect("no element in dragon")
            .trim_end()
            .to_string();
        let name = split_name
            .clone()
            .join("")
            .trim_end_matches(" (")
            .to_string();
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
    pub item_list: Option<HashSet<String>>,
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
            item_list: None,
        }
    }
    fn calc_item_count(&mut self) {
        self.item_count = self.da_count + self.dc_count + self.nda_count + self.artifact_count
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
            Some(dmk) => format!("**{}**\n", dmk.to_string()),
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
            _ => format!("**War Waves:** {}\n", self.wars.total_waves_string()),
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
            "**DF ID: **[{}]({}{})\n{}{}{}{}{}{}{}{}{}{}{}{}",
            id,
            CHARPAGE,
            id,
            self.get_da_str(),
            self.get_dmk_str(),
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
pub struct WarBuilder {
    pub warlabel: Option<String>,
    pub war_text: Option<String>,
}
impl WarBuilder {
    pub fn default() -> WarBuilder {
        WarBuilder {
            warlabel: None,
            war_text: None,
        }
    }
    pub fn build(self) -> War {
        War::new(
            self.warlabel.expect("warlabel is None"),
            self.war_text.expect("wartext is None"),
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
    pub fn new(warlabel: String, war_text: String) -> War {
        let mut war_split = war_text.splitn(2, ',');
        let waves = war_split.next().map(|s| s.trim().to_string()).unwrap();
        let rares = war_split.next().map(|s| s.trim().to_string()).unwrap();
        War {
            warlabel,
            waves,
            rares,
        }
    }
    pub fn war_string(&self) -> String {
        format!("**{}**\n*{} ,{}*\n", self.warlabel, self.waves, self.rares)
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
    pub fn calc_waves_cleared(&self) -> u32 {
        self.war_list.iter().fold(0, |c, x| {
            x.waves.replace(" waves", "").parse::<u32>().unwrap() + c
        })
    }
    pub fn is_empty(&self) -> bool {
        self.war_list.is_empty()
    }
    pub fn vec_of_war_strings(&self) -> Vec<String> {
        self.war_list.iter().map(|w| w.war_string()).collect()
    }
    pub fn total_waves_string(&self) -> String {
        self.calc_waves_cleared().to_formatted_string(&Locale::en)
    }
    pub fn wars(&self)->&Vec<War>{
        &self.war_list
    }
}
pub fn parse_df_character(document: &Html) -> LookupState {
    let mut character = DFCharacterData::default();
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };
    character.name = match charpagedetails.select(&h1_selector).next() {
        None => return parse_df_character_flash(document),
        Some(name) => name.text().collect::<Vec<_>>().join(" ").trim().to_string(),
    };

    let cb_label_selector = Selector::parse("div#charpagedetails .card-body label").unwrap();
    for label in document.select(&cb_label_selector) {
        let label_text = label.text().collect::<Vec<_>>().join("");
        match label_text.as_str() {
            "Dragon Amulet Owner" => {
                character.dragon_amulet = true;
            }
            "Gold:" => {
                character.gold = label
                    .next_sibling()
                    .unwrap()
                    .value()
                    .as_text()
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .unwrap_or(0);
            }
            "Level:" => {
                character.level = label
                    .next_sibling()
                    .unwrap()
                    .value()
                    .as_text()
                    .unwrap()
                    .trim()
                    .parse::<u8>()
                    .ok()
                    .unwrap_or(0)
            }
            "Dragon:" => {
                let dragon_name_element = label.next_siblings().nth(1).unwrap();
                let dragon_name_text = ElementRef::wrap(dragon_name_element)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                character.dragon = Some(Dragon::new(dragon_name_text));
            }
            "Last Played:" => {
                character.last_played = NaiveDate::parse_from_str(
                    label
                        .next_sibling()
                        .unwrap()
                        .value()
                        .as_text()
                        .unwrap()
                        .trim(),
                    "%A, %B %d, %Y",
                )
                .unwrap_or_default();
            }
            "Doom Knight" | "Master Doom Knight" | "Superior Doom Knight" | "Elite Doom Knight" => {
                character.dmk = Some(label_text)
            }
            _ => {}
        }
    }
    let mut unique_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let item_selector = Selector::parse("div#charpagedetails.card-columns.mx-auto span").unwrap();
    let mut warbuilder = WarBuilder::default();
    for span in document.select(&item_selector).into_iter() {
        let mut item = true;
        let item_name = span.text().next().unwrap();
        let mut classes = span.value().classes();
        if let Some(class) = classes.next() {
            match class {
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
                    warbuilder.warlabel = Some(item_name.to_owned());
                }
                "d-inline-block" => {
                    item = false;
                    warbuilder.war_text = Some(item_name.to_owned());
                    character.wars.push_war(warbuilder.build());
                    warbuilder = WarBuilder::default();
                }
                _ => {
                    dbg!(span.value().classes().collect::<Vec<_>>());
                    dbg!(item_name);
                    dbg!(class);
                    panic!("UnexpectedItemType");
                }
            }
            if item {
                unique_names.insert(item_name.to_owned());
            }
        }
    }

    character.unique_item_count = unique_names.len() as u16;
    character.calc_item_count();
    LookupState::CharacterPage(character)
}
pub fn parse_df_character_wars_only(document: &Html) -> LookupState {
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };
    let flashvars_selector = Selector::parse(r#"param[name="FlashVars"]"#).unwrap();
    let character_name = match document.select(&flashvars_selector).next() {
        Some(vars) => vars
            .value()
            .attr("value")
            .unwrap()
            .split_once(" ")
            .unwrap()
            .0
            .trim()[5..]
            .to_string(),
        None => {
            let h1_selector = Selector::parse("h1").unwrap();
            charpagedetails
                .select(&h1_selector)
                .next()
                .expect("No Name Found")
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        }
    };
    let mut wars = WarList::new();

    let war_label = Selector::parse("span.warlabel").unwrap();
    let war_text = Selector::parse("span.mx-2").unwrap();
    let war_text = document.select(&war_text);
    let wars_s = document.select(&war_label).zip(war_text);
    wars_s.for_each(|(warlabel, waves)| {
        wars.push_war(War::new(
            warlabel.text().next().unwrap().to_owned(),
            waves.text().next().unwrap().to_owned(),
        ))
    });
    LookupState::Wars(character_name, wars)
}
pub fn parse_df_character_inventory_only(document: &Html) -> LookupState {
    let mut items = Vec::new();
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };
    let flashvars_selector = Selector::parse(r#"param[name="FlashVars"]"#).unwrap();
    let character_name = match document.select(&flashvars_selector).next() {
        Some(vars) => vars
            .value()
            .attr("value")
            .unwrap()
            .split_once(" ")
            .unwrap()
            .0
            .trim()[5..]
            .to_string(),
        None => {
            let h1_selector = Selector::parse("h1").unwrap();
            charpagedetails
                .select(&h1_selector)
                .next()
                .expect("No Name Found")
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        }
    };
    let div_card_selector = Selector::parse("div.card").unwrap();
    let span_selector = Selector::parse("span").unwrap();
    let h4_selector = Selector::parse("h4").unwrap();
    for card in document.select(&div_card_selector) {
        if let Some(h4) = card.select(&h4_selector).next() {
            if h4.text().collect::<String>() == "Inventory" {
                card.select(&span_selector)
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, x)| {
                        let item_name = x.inner_html();
                        match x.value().classes().next().expect("no classes") {
                            "coins" => items.push(format!("{}: **__{}__**", i, item_name)),
                            "amulet" => items.push(format!("{}: **{}**", i, item_name)),
                            "artifact" => items.push(format!("{}: *{}*", i, item_name)),
                            "gold" => items.push(format!("{}: {}", i, item_name)),
                            _ => panic!("UnexpectedItemClass"),
                        };
                    });
            }
            break;
        }
    }
    LookupState::Inventory(character_name, items)
}
pub fn parse_df_character_duplicates(document: &Html) -> LookupState {
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };
    let mut items = HashMap::new();
    let flashvars_selector = Selector::parse(r#"param[name="FlashVars"]"#).unwrap();
    let character_name = match document.select(&flashvars_selector).next() {
        Some(vars) => vars
            .value()
            .attr("value")
            .unwrap()
            .split_once(" ")
            .unwrap()
            .0
            .trim()[5..]
            .to_string(),
        None => {
            let h1_selector = Selector::parse("h1").unwrap();
            charpagedetails
                .select(&h1_selector)
                .next()
                .expect("No Name Found")
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        }
    };
    let item_selector = Selector::parse("div#charpagedetails.card-columns.mx-auto span").unwrap();
    for span in document.select(&item_selector).into_iter() {
        let item_name = span.text().next().unwrap();
        let mut classes = span.value().classes();
        if let Some(class) = classes.next() {
            match class {
                "gold" | "coins" | "amulet" | "artifact" => {
                    if let Some(occurrence) = items.get_mut(item_name) {
                        *occurrence += 1;
                    } else {
                        items.insert(item_name.to_string(), 1);
                    }
                }
                "warlabel" | "d-inline-block" => (),
                _ => {
                    panic!("UnexpectedItemType");
                }
            }
        }
    }
    items.retain(|_, &mut v| v > 1);
    LookupState::Duplicates(character_name, items)
}

pub fn parse_df_character_flash(document: &Html) -> LookupState {
    let flashvars_selector = Selector::parse(r#"param[name="FlashVars"]"#).unwrap();
    let flashvars = match document.select(&flashvars_selector).next() {
        Some(vars) => vars.value().attr("value").unwrap(),
        None => return parse_df_character(document),
    };
    let key_value_pairs: Vec<&str> = flashvars.split('&').collect();
    let mut flashvars_map = std::collections::HashMap::new();
    for pair in key_value_pairs {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            flashvars_map.insert(key.to_string(), value.to_string());
        }
    }
    LookupState::FlashCharatcerPage(flashvars_map)
}
fn hex(color_name: &str, value: &String) -> String {
    let hex = value.parse::<i32>().unwrap_or_default();
    format!(
        "**{}: ** [{:x}]({}{:x})\n",
        color_name, hex, COLOR_SITE, hex
    )
}
pub fn get_discord_embed_description_flash(
    flashdata: HashMap<String, String>,
    df_id: i32,
) -> String {
    let up = format!("**\"up\":** {}", flashdata.get("up").unwrap());
    let df_id = format!("**DF ID:** [{}]({}{})\n", df_id, CHARPAGE, df_id);
    let dragon_str = match flashdata.get("NoDragon").unwrap().as_str() {
        "wrong" => {
            let dragon_eye_color = hex("Dragon Eye Color", flashdata.get("DeyeC").unwrap());
            let dragon_wing_color = hex("Dragon Wing Color", flashdata.get("DwingC").unwrap());
            let dragon_skin_color = hex("Dragon Skin Color", flashdata.get("DskinC").unwrap());
            let dragon_horn_color = hex("Dragon Horn Color", flashdata.get("DhornC").unwrap());
            format!(
                "{}{}{}{}",
                dragon_skin_color, dragon_eye_color, dragon_horn_color, dragon_wing_color
            )
        }
        "right" => "".to_string(),
        _ => {
            panic!("dragon not right/wrong {}", df_id)
        }
    };
    let classname = format!("**Class:** {}\n", flashdata.get("ClassName").unwrap());
    let trim_color = hex("Trim Color", flashdata.get("TrimColor").unwrap());
    let last_played = format!(
        "**Last Played**: {}\n",
        flashdata.get("LastPlayed").unwrap()
    );
    let created = format!("**Created:** {}\n", flashdata.get("Created").unwrap());
    let gender = format!(
        "**Gender:** {}\n",
        match flashdata.get("Gender").unwrap().as_str() {
            "M" => "Male".to_owned(),
            "F" => "Female".to_owned(),
            _ => panic!("Unknown Gender"),
        }
    );
    let founder = match flashdata.get("Founder").unwrap().as_str() {
        "1" => "**Founder**\n",
        _ => "",
    };
    let race = format!("**Race:** {}\n", flashdata.get("Race").unwrap());
    let base_color = hex("Base Color:", flashdata.get("BaseColor").unwrap());
    let skin_color = hex("Skin Color", flashdata.get("SkinColor").unwrap());
    let dragon_amulet = format!("**DA:** {}\n", flashdata.get("DA").unwrap());
    let hair_color = hex("Hair Color", flashdata.get("HairColor").unwrap());
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        df_id,
        founder,
        dragon_amulet,
        classname,
        last_played,
        created,
        race,
        gender,
        hair_color,
        skin_color,
        base_color,
        trim_color,
        dragon_str,
        up
    )
}
pub fn parse_df_character_with_items(document: &Html) -> LookupState {
    let mut character = DFCharacterData::default();
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };

    match charpagedetails.select(&h1_selector).next() {
        None => {
            if let LookupState::FlashCharatcerPage(chardata) = parse_df_character_flash(document) {
                character.name = chardata.get("Name").unwrap().to_string();
                character.gold = chardata.get("Gold").unwrap().parse::<i32>().unwrap();
                character.level = chardata.get("Level").unwrap().parse::<u8>().unwrap();
                character.last_played =
                    NaiveDate::parse_from_str(chardata.get("LastPlayed").unwrap(), "%Y/%m/%d")
                        .unwrap();
            } else {
                return LookupState::NotFound;
            }
        }
        Some(name) => {
            let cb_label_selector =
                Selector::parse("div#charpagedetails .card-body label").unwrap();
            for label in document.select(&cb_label_selector) {
                let label_text = label.text().collect::<Vec<_>>().join("");
                match label_text.as_str() {
                    "Dragon Amulet Owner" => {
                        character.dragon_amulet = true;
                    }
                    "Gold:" => {
                        character.gold = label
                            .next_sibling()
                            .unwrap()
                            .value()
                            .as_text()
                            .unwrap()
                            .trim()
                            .parse::<i32>()
                            .ok()
                            .unwrap_or(0);
                    }
                    "Level:" => {
                        character.level = label
                            .next_sibling()
                            .unwrap()
                            .value()
                            .as_text()
                            .unwrap()
                            .trim()
                            .parse::<u8>()
                            .ok()
                            .unwrap_or(0)
                    }
                    "Dragon:" => {
                        let dragon_name_element = label.next_siblings().nth(1).unwrap();
                        let dragon_name_text = ElementRef::wrap(dragon_name_element)
                            .unwrap()
                            .text()
                            .collect::<Vec<_>>()
                            .join("")
                            .trim()
                            .to_string();
                        character.dragon = Some(Dragon::new(dragon_name_text));
                    }
                    "Last Played:" => {
                        character.last_played = NaiveDate::parse_from_str(
                            label
                                .next_sibling()
                                .unwrap()
                                .value()
                                .as_text()
                                .unwrap()
                                .trim(),
                            "%A, %B %d, %Y",
                        )
                        .unwrap_or_default();
                    }
                    "Doom Knight"
                    | "Master Doom Knight"
                    | "Superior Doom Knight"
                    | "Elite Doom Knight" => character.dmk = Some(label_text),
                    _ => {}
                }
            }
            character.name = name.text().collect::<Vec<_>>().join(" ").trim().to_string();
        }
    };

    let mut unique_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let item_selector = Selector::parse("div#charpagedetails.card-columns.mx-auto span").unwrap();
    let mut warbuilder = WarBuilder::default();
    for span in document.select(&item_selector).into_iter() {
        let mut item = true;
        let item_name = span.text().next().unwrap();
        let mut classes = span.value().classes();
        if let Some(class) = classes.next() {
            match class {
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
                    warbuilder.warlabel = Some(item_name.to_owned());
                }
                "d-inline-block" => {
                    item = false;
                    warbuilder.war_text = Some(item_name.to_owned());
                    character.wars.push_war(warbuilder.build());
                    warbuilder = WarBuilder::default();
                }
                _ => {
                    dbg!(span.value().classes().collect::<Vec<_>>());
                    dbg!(item_name);
                    dbg!(class);
                    panic!("UnexpectedItemType");
                }
            }
            if item {
                unique_names.insert(item_name.to_owned());
            }
        }
    }

    character.unique_item_count = unique_names.len() as u16;
    character.calc_item_count();
    LookupState::CharacterPage(character)
}
