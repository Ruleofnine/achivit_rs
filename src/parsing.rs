use crate::lookup_df::LookupCategory;
use crate::lookup_df::LookupState;
use crate::requests::{
    fetch_page_with_user_agent, CHARPAGE, COLOR_SITE, FLASH_USER_AGENT, USER_AGENT,
};
use chrono::NaiveDate;
use color_eyre::Result;
use getset::Getters;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::fs;
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParsingCategory {
    CharacterPage,
    FlashCharacterPage,
    Wars,
    Items,
    Inventory,
    Duplicates,
    Compare,
    Roles,
    Ascendancies,
}
pub trait IsFlash {
    fn is_flash(&self) -> bool;
}
impl IsFlash for ParsingCategory {
    fn is_flash(&self) -> bool {
        matches!(self,ParsingCategory::FlashCharacterPage)
    }
}

impl IsFlash for LookupCategory {
    fn is_flash(&self) -> bool {
        matches!(self, LookupCategory::FlashCharacterPage)
    }
}
impl From<&LookupCategory> for ParsingCategory {
    fn from(item: &LookupCategory) -> Self {
        match item {
            LookupCategory::CharacterPage => ParsingCategory::CharacterPage,
            LookupCategory::FlashCharacterPage => ParsingCategory::FlashCharacterPage,
            LookupCategory::Inventory => ParsingCategory::Inventory,
            LookupCategory::Wars => ParsingCategory::Wars,
            LookupCategory::Duplicates => ParsingCategory::Duplicates,
            LookupCategory::Roles => ParsingCategory::Roles,
            LookupCategory::Ascendancies => ParsingCategory::Ascendancies,
        }
    }
}

pub struct FileFetcher<'a> {
    file_path: &'a str,
    category: ParsingCategory,
}
impl<'a> FileFetcher<'a> {
    pub fn new(file_path: &str) -> FileFetcher {
        FileFetcher {
            file_path,
            category: ParsingCategory::CharacterPage,
        }
    }
    pub fn category(&'a mut self, category: ParsingCategory) -> &mut FileFetcher {
        self.category = category;
        self
    }
    pub async fn fetch_data(&'a self) -> Result<CharacterData> {
        let str = fs::read_to_string(self.file_path.to_string()).await?;
        Ok(CharacterData {
            str,
            df_id: 0,
            category: self.category,
        })
    }
}

pub struct CharacterFetcher {
    df_id: i32,
    category: ParsingCategory,
}
impl CharacterFetcher {
    pub fn new(df_id: i32, category: LookupCategory) -> CharacterFetcher {
        CharacterFetcher {
            df_id,
            category: ParsingCategory::from(&category),
        }
    }
    pub fn url(&self) -> String {
        format!("{}{}", CHARPAGE, self.df_id)
    }
    pub fn user_agent(&self) -> &str {
        match self.category {
            ParsingCategory::FlashCharacterPage => FLASH_USER_AGENT,
            _ => USER_AGENT,
        }
    }
    pub fn category(mut self, category: ParsingCategory) -> CharacterFetcher {
        self.category = category;
        self
    }
    pub fn fetch_data(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<CharacterData>> + Send + 'static>> {
        Box::pin(async move {
            let str = fetch_page_with_user_agent(self.user_agent(), &self.url()).await?;
            let category = self.category;
            let df_id = self.df_id;
            Ok(CharacterData {
                str,
                df_id,
                category,
            })
        })
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
pub struct CharacterData {
    str: String,
    df_id: i32,
    category: ParsingCategory,
}
impl CharacterData {
    pub fn to_lookupstate(&self) -> Result<LookupState> {
        let document = Html::parse_document(&self.str);
        Ok(match self.category {
            ParsingCategory::CharacterPage => parse_df_character(&document, *self.df_id()),
            ParsingCategory::FlashCharacterPage => {
                parse_df_character_flash(&document, *self.df_id())
            }
            ParsingCategory::Wars => parse_df_character_wars_only(&document, *self.df_id()),
            ParsingCategory::Duplicates => parse_df_character_duplicates(&document, *self.df_id()),
            ParsingCategory::Inventory => {
                parse_df_character_inventory_only(&document, *self.df_id())
            }
            ParsingCategory::Items => {
                parse_df_character_with_items(&document, self.category, *self.df_id())
            }
            ParsingCategory::Compare => {
                parse_df_character_with_items(&document, self.category, *self.df_id())
            }
            ParsingCategory::Roles => {
                parse_df_character_with_items(&document, self.category, *self.df_id())
            }
            ParsingCategory::Ascendancies => {
                parse_df_character_with_items(&document, self.category, *self.df_id())
            }
        })
    }
}

pub fn convert_html_to_discord_format(input: &str) -> String {
    let re = Regex::new(r#"<a href="(?P<url>[^"]+)"[^>]*>(?P<text>[^<]+)</a>"#).unwrap();
    re.replace_all(input, "[${text}](${url})").to_string()
}
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Clone)]
pub struct Dragon {
    pub name: String,
    pub dragon_type: String,
}

impl Dragon {
    fn new(mut dragon_str: String) -> Dragon {
        dragon_str.truncate(dragon_str.len() - 7);
        let mut split_name: Vec<_> = dragon_str.split_inclusive('(').collect();
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
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ItemTag {
    NDA,
    DA,
    DC,
    ARTIFACT,
}
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Item {
    tag: ItemTag,
    stackable: bool,
    amount: i32,
}
impl Item {
    fn new(tag: ItemTag, stackable: bool, amount: i32) -> Item {
        Item {
            tag,
            stackable,
            amount,
        }
    }
}
#[derive(Getters)]
#[getset(get = "pub", get_mut = "pub")]
#[derive(Debug,Default)]
pub struct Items {
    items: HashMap<String, Item>,
}
impl Items  {
    fn new_item(&mut self, name: String, tag: ItemTag, stackable: bool, amount: i32) {
        let item = Item::new(tag, stackable, amount);
        self.insert(name, item);
    }
    pub fn items_mut(&mut self) -> &mut HashMap<String, Item> {
        &mut self.items
    }
    pub fn contains(&self, item: &String) -> bool {
        self.items().contains_key(item)
    }
    pub fn dups(&self) -> bool {
        self.items()
            .iter()
            .any(|i| i.1.amount > 1 && !i.1.stackable())
    }
    pub fn split_list(&self) -> impl Iterator<Item = Vec<&String>> {
        let mut da_iter = Vec::new();
        let mut dc_iter = Vec::new();
        let mut nda_iter = Vec::new();
        let mut artifact_iter = Vec::new();
        self.items().iter().for_each(|(name, item)| match item.tag {
            ItemTag::NDA => {
                nda_iter.push(name);
            }
            ItemTag::DA => {
                da_iter.push(name);
            }
            ItemTag::DC => {
                dc_iter.push(name);
            }
            ItemTag::ARTIFACT => {
                artifact_iter.push(name);
            }
        });
        vec![nda_iter, da_iter, dc_iter, artifact_iter].into_iter()
    }
    fn insert(&mut self, name: String, item: Item) {
        match self.items.get_mut(&name) {
            Some(item) => {
                item.amount += 1;
            }
            None => {
                self.items.insert(name, item);
            }
        };
    }
    pub fn text(num: usize) -> String {
        let str = match num {
            0 => "NDA Items",
            1 => "DA Items",
            2 => "DC Items",
            _ => "Artifacts",
        };
        str.to_string()
    }
    pub fn new() -> Items {
        Items {
            items: HashMap::new(),
        }
    }
    pub fn count(&self) -> u16 {
        self.items.len() as u16
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug)]
pub struct DFCharacterData {
    pub id: i32,
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
    pub item_list: Option<Items>,
}
impl DFCharacterData {
    fn default(df_id: i32) -> DFCharacterData {
        DFCharacterData {
            id: df_id,
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
    fn calc_item_count(&mut self) -> u16 {
        self.da_count + self.dc_count + self.nda_count + self.artifact_count
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
            Some(dmk) => format!("**{dmk}**\n"),
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

#[derive(Debug,Default)]
pub struct WarBuilder {
    pub warlabel: Option<String>,
    pub war_text: Option<String>,
}

impl WarBuilder {
    pub fn build(self) -> War {
        War::new(
            self.warlabel.expect("warlabel is None"),
            self.war_text.expect("wartext is None"),
        )
    }
}
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Clone)]
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
    pub fn waves_int(&self) -> i32 {
        self.waves().replace(" waves", "").parse().unwrap()
    }
}

#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Clone)]
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
    pub fn calc_waves_cleared(&self) -> i32 {
        self.war_list.iter().fold(0, |c, x| {
            x.waves.replace(" waves", "").parse::<i32>().unwrap() + c
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
    pub fn wars(&self) -> &[War] {
        &self.war_list
    }
}
pub fn parse_df_character(document: &Html, df_id: i32) -> LookupState {
    let mut character = DFCharacterData::default(df_id);
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };
    character.name = match charpagedetails.select(&h1_selector).next() {
        None => return parse_df_character_flash(document, df_id),
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
    let mut unique: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let item_selector = Selector::parse("div#charpagedetails.card-columns.mx-auto span").unwrap();
    let mut warbuilder = WarBuilder::default();
    for span in document.select(&item_selector) {
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
                unique.insert(item_name.to_owned());
            }
        }
    }

    character.unique_item_count = unique.len() as u16;
    character.calc_item_count();
    LookupState::CharacterPage(character)
}
pub fn parse_df_character_wars_only(document: &Html, _df_id: i32) -> LookupState {
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
            .split_once(' ')
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
pub fn parse_df_character_inventory_only(document: &Html, _df_id: i32) -> LookupState {
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
            .split_once(' ')
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
                card.select(&span_selector).enumerate().for_each(|(i, x)| {
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
pub fn parse_df_character_duplicates(document: &Html, _df_id: i32) -> LookupState {
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
            .split_once(' ')
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
    for span in document.select(&item_selector) {
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

pub fn parse_df_character_flash(document: &Html, df_id: i32) -> LookupState {
    let flashvars_selector = Selector::parse(r#"param[name="FlashVars"]"#).unwrap();
    let flashvars = match document.select(&flashvars_selector).next() {
        Some(vars) => vars.value().attr("value").unwrap(),
        None => return parse_df_character(document, df_id),
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
fn hex(color_name: &str, value: &str) -> String {
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
pub fn parse_df_character_with_items(
    document: &Html,
    category: ParsingCategory,
    df_id: i32,
) -> LookupState {
    let mut character = DFCharacterData::default(df_id);
    let charpage_selector = Selector::parse("div#charpagedetails").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();
    let charpagedetails = match document.select(&charpage_selector).next() {
        Some(charpagedetails) => charpagedetails,
        None => return LookupState::NotFound,
    };

    match charpagedetails.select(&h1_selector).next() {
        None => {
            // if let LookupState::FlashCharatcerPage(chardata) =
            // parse_df_character_flash(document, df_id)
            // {
            // character.name = chardata.get("Name").unwrap().to_string();
            // character.gold = chardata.get("Gold").unwrap().parse::<i32>().unwrap();
            // character.level = chardata.get("Level").unwrap().parse::<u8>().unwrap();
            // dbg!(chardata.get("LastPlayed"));
            // character.last_played =
            //     NaiveDate::parse_from_str(chardata.get("LastPlayed").unwrap(), "%m/%d/%Y").expect("failed to parse date");
            // } else {
            return LookupState::NotFound;
            // }
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
    let mut items = Items::default();
    let item_selector = Selector::parse("div#charpagedetails.card-columns.mx-auto span").unwrap();
    let mut warbuilder = WarBuilder::default();
    for span in document.select(&item_selector) {
        let item_name = span.text().next().unwrap();
        let (item_name, amount, stackable) = match item_name.split_once(" (x") {
            None => (item_name.to_string(), 1, false),
            Some(item) => {
                let x_str = item.1;
                let amount = x_str[..x_str.len() - 1]
                    .parse::<i32>()
                    .expect("failed to parse stack amount");
                (item.0.to_string(), amount, true)
            }
        };
        let mut classes = span.value().classes();
        if let Some(class) = classes.next() {
            match class {
                "gold" => {
                    character.nda_count += 1;
                    items.new_item(item_name, ItemTag::NDA, stackable, amount);
                }
                "coins" => {
                    character.dc_count += 1;
                    items.new_item(item_name, ItemTag::DC, stackable, amount);
                }
                "amulet" => {
                    character.da_count += 1;
                    items.new_item(item_name, ItemTag::DA, stackable, amount);
                }
                "artifact" => {
                    character.artifact_count += 1;
                    items.new_item(item_name, ItemTag::ARTIFACT, stackable, amount);
                }
                "warlabel" => {
                    warbuilder.warlabel = Some(item_name);
                }
                "d-inline-block" => {
                    warbuilder.war_text = Some(item_name);
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
        }
    }
    character.unique_item_count = items.count();
    character.item_count = character.calc_item_count();
    character.item_list = Some(items);
    match category {
        ParsingCategory::Roles => LookupState::Roles(character),
        ParsingCategory::Ascendancies => LookupState::Ascendancies(character),
        _ => LookupState::CharacterPage(character),
    }
}
