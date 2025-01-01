use crate::parsing::{DFCharacterData, Items, WarList};
use color_eyre::Result;
use serde_derive::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use crate::{Context, Error};
use crate::lookup_df::LookupCategory;
use crate::manage_users::autocomplete_character;
use crate::paginate::{get_requirement_pages, paginate, PaginateEmbed};
use crate::parsing::{CharacterFetcher, ParsingCategory};
use crate::db::INN_GUILD_ID;
use crate::rng::random_rgb;
use crate::serenity::{Color, User};

pub enum RequirementListType {
    Roles,
    Ascend,
}
fn req_type_item() -> ReqType {
    ReqType::Item
}
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Requirement {
    pub name: String,
    pub description: Option<String>,
    pub prereqs: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default = "req_type_item")]
    pub req_type: ReqType,
    pub amount: Option<i32>,
}
impl Requirement {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn description(&self) -> &String {
        self.description
            .as_ref()
            .unwrap_or_else(|| panic!("Role: {} Expected 'description'", self.name()))
    }
    pub fn prereqs(&self) -> &[String] {
        self.prereqs
            .as_ref()
            .unwrap_or_else(|| panic!("Role: {} Expected 'prereqs'", self.name()))
    }
    pub fn required(&self) -> &[String] {
        self.required
            .as_ref()
            .unwrap_or_else(|| panic!("Role: {} Expected 'required'", self.name()))
    }
    pub fn amount(&self) -> i32 {
        self.amount
            .unwrap_or_else(|| panic!("Role: {} Expected 'amount'", self.name()))
    }
}
fn max_last(a: &Requirement, b: &Requirement) -> Ordering {
    match (&a.req_type, &b.req_type) {
        (ReqType::Max, ReqType::Max) => Ordering::Equal,
        (ReqType::Max, _) => Ordering::Greater,
        (_, ReqType::Max) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RequirementList(Vec<Requirement>);
impl RequirementList {
    pub fn requirements(&self) -> &[Requirement] {
        &self.0
    }
    fn sort(&mut self) {
        self.0.sort_by(max_last)
    }
    pub fn sort_alphabetical(&mut self) {
        self.0.sort_by(|a, b| a.name().cmp(b.name()))
    }
}
#[derive(Serialize, Debug, Deserialize, Eq, PartialEq, Hash)]
#[allow(non_snake_case)]
#[serde(rename_all = "PascalCase")]
pub enum ReqType {
    Wars,
    Waves,
    Item,
    Gold,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "Item/Amount")]
    ItemAmount,
    #[serde(rename = "Item/Unique")]
    ItemUnique,
    #[serde(rename = "Item/Lean")]
    ItemLean,
    #[serde(rename = "Item/DC")]
    ItemDC,
    #[serde(rename = "Item/Stackable")]
    ItemStackable,
    Inn,
}

impl fmt::Display for ReqType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReqType::Wars => write!(f, "Wars"),
            ReqType::Waves => write!(f, "Waves"),
            ReqType::Item => write!(f, "Item"),
            ReqType::Gold => write!(f, "Gold"),
            ReqType::ItemLean => write!(f, "Item/Lean"),
            ReqType::ItemDC => write!(f, "Item/DC"),
            ReqType::ItemAmount => write!(f, "Item/Amount"),
            ReqType::Max => write!(f, "Max"),
            ReqType::ItemStackable => write!(f, "Item/Stackable"),
            ReqType::Inn => write!(f, "Inn"),
            ReqType::ItemUnique => write!(f, "Item/Unique"),
        }
    }
}

impl ReqType {
    fn from_str(s: &str) -> Option<ReqType> {
        match s {
            "Wars" => Some(ReqType::Wars),
            "Item/Lean" => Some(ReqType::ItemLean),
            "Item/DC" => Some(ReqType::ItemDC),
            "Waves" => Some(ReqType::Waves),
            "Item/Amount" => Some(ReqType::ItemAmount),
            "Max" => Some(ReqType::Max),
            "Item/Stackable" => Some(ReqType::ItemStackable),
            "Item" => Some(ReqType::Item),
            "Gold" => Some(ReqType::Gold),
            "Inn" => Some(ReqType::Inn),
            "Item/Unique" => Some(ReqType::ItemUnique),
            _ => None, // Return None if the string does not match any variant
        }
    }
}
pub fn get_requirements_file(path: &str) -> Result<RequirementList> {
    let file = File::open(format!("JSONS/{path}"))?;
    let reader = BufReader::new(file);
    let mut roles: RequirementList = serde_json::from_reader(reader)?;
    roles.sort();
    Ok(roles)
}

pub async fn get_requirements(guild_id: i64, pool: &PgPool) -> Result<RequirementList> {
    let reqs = query!(
        "select * from requirements where guild_id = $1 order by name",
        guild_id
    )
    .fetch_all(pool)
    .await?;
    let mut requirements: RequirementList = RequirementList(vec![]);
    for req in reqs {
        let name = req.name;
        let description = req.description;
        let req_id = req.requirementid;
        let req_type = ReqType::from_str(&req.r#type).unwrap();
        let amount = req.amount;
        let prereq_records = query!(
            "select prerequisiterequirementid from prerequisites where RequirementId = $1",
            req.requirementid
        )
        .fetch_all(pool)
        .await?;
        let items = query!(
            "select itemname from requireditems where requirementid = $1",
            req_id
        )
        .fetch_all(pool)
        .await?
        .iter()
        .map(|r| r.itemname.clone())
        .collect::<Vec<String>>();
        let prereqs = if prereq_records.is_empty() {
            None
        } else {
            let mut prereqs = vec![];
            for prereq_id in prereq_records {
                let name = query!(
                    "select name from requirements where requirementid = $1",
                    prereq_id.prerequisiterequirementid
                )
                .fetch_one(pool)
                .await?;
                prereqs.push(name.name);
            }
            Some(prereqs)
        };
        let required = if items.is_empty() { None } else { Some(items) };
        requirements.0.push(Requirement {
            name,
            description,
            required,
            req_type,
            prereqs,
            amount,
        })
    }
    requirements.sort();
    Ok(requirements)
}

pub fn get_requirements_bytes(bytes: &[u8]) -> Result<RequirementList> {
    let mut roles: RequirementList = serde_json::from_slice(bytes)?;
    roles.sort();
    Ok(roles)
}
fn check_item(role: &Requirement, char_items: &Items) -> bool {
    let items = role
        .required
        .as_ref()
        .expect("Item Role requires item list");
    items.iter().all(|i| char_items.contains(i))
}
fn check_war(role: &Requirement, char: &DFCharacterData) -> bool {
    let amount = role.amount.expect("War needs amount") as usize;
    char.wars.wars().len() >= amount
}
fn check_item_amount(role: &Requirement, char_items: &Items) -> bool {
    let amount = role.amount();
    let items = role.required();
    let count = items.iter().filter(|&i| char_items.contains(i)).count() as i32;
    count >= amount
}

fn check_waves(role: &Requirement, wars: &WarList) -> bool {
    let amount = role.amount();
    wars.war_list().iter().any(|w| w.waves_int() >= amount)
}
fn check_gold(role: &Requirement, gold: &i32) -> bool {
    let amount = role.amount();
    *gold >= amount
}
fn check_max_role(roles: &[Requirement], role: &Requirement, aquired_roles: &[usize]) -> bool {
    let prereqs = role.prereqs();
    let amount = prereqs.len();
    let mut has = 0;
    for index in aquired_roles {
        let role = roles.get(*index).as_ref().expect("expeced prereqs").name();
        if prereqs.contains(role) {
            has += 1
        };
        if has >= amount {
            return true;
        }
    }
    false
}
fn check_item_stackable(role: &Requirement, items: &Items) -> bool {
    let required: Vec<(String, i32)> = role
        .required()
        .iter()
        .map(|i| {
            let split_item = i.split_once(" (x");
            let name = split_item
                .as_ref()
                .expect("expected item in stackable")
                .0
                .to_string();
            let amount = split_item
                .as_ref()
                .expect("Expected Stackable Item to have (x ..))")
                .1
                .trim_end_matches(')')
                .parse()
                .expect("failed to parse stackable amount");
            (name, amount)
        })
        .collect();
    required.iter().all(|(req_item, req_amount)| {
        items
            .items()
            .iter()
            .any(|(name, item)| name == req_item && item.amount() >= req_amount)
    })
}
fn check_all_inn_reqs(items: &Items) -> bool {
    let list = get_requirements_file("InnList.json").expect("failed to get in list");
    list.requirements()
        .iter()
        .all(|innreq| innreq.required().iter().all(|i| items.contains(i)))
}
fn aquired_roles_indexes(roles: &mut RequirementList, char: &DFCharacterData) -> Vec<usize> {
    let char_items = char.item_list.as_ref().expect("expected char items");
    let dups = char_items.dups();
    let roles_list = roles.requirements();
    let mut roles_indexes_to_remove: Vec<usize> = vec![];
    for (i, role) in roles_list.iter().enumerate() {
        let aquired = match role.req_type {
            ReqType::Wars => check_war(role, char),
            ReqType::Item => check_item(role, char_items),
            ReqType::ItemAmount => check_item_amount(role, char_items),
            ReqType::Waves => check_waves(role, char.wars()),
            ReqType::Gold => check_gold(role, char.gold()),
            ReqType::Max => check_max_role(roles_list, role, &roles_indexes_to_remove),
            ReqType::ItemUnique => role.amount() as u16 <= *char.unique_item_count(),
            ReqType::ItemLean => role.amount() as u16 <= *char.unique_item_count() && !dups,
            ReqType::ItemDC => role.amount() as u16 <= *char.dc_count(),
            ReqType::ItemStackable => check_item_stackable(role, char_items),
            ReqType::Inn => {
                check_max_role(roles_list, role, &roles_indexes_to_remove)
                    && check_all_inn_reqs(char_items)
            }
        };
        if aquired {
            roles_indexes_to_remove.push(i);
        }
    }
    roles_indexes_to_remove
}
fn prereq_roles_to_remove(roles: &[Requirement]) -> Vec<usize> {
    let mut prereq_roles = Vec::new();
    for role in roles {
        if let Some(prereqs) = &role.prereqs {
            for prereq in prereqs {
                for (i, checking_role) in roles.iter().enumerate() {
                    if prereq == checking_role.name() && !prereq_roles.contains(&i) {
                        prereq_roles.push(i);
                    }
                }
            }
        }
    }
    prereq_roles
}
pub async fn check_requirements(
    char: &DFCharacterData,
    guild_id: i64,
    pool: &PgPool,
) -> Result<RequirementList> {
    let mut roles = get_requirements(guild_id, pool).await?;
    let mut aquired_roles = aquired_roles_indexes(&mut roles, char);
    aquired_roles.sort_by(|a, b| b.cmp(a));
    let mut roles: Vec<Requirement> = aquired_roles
        .iter()
        .map(|i| roles.0.swap_remove(*i))
        .collect();
    aquired_roles.sort_by(|a, b| b.cmp(a));
    let mut prereq_roles = prereq_roles_to_remove(&roles);
    prereq_roles.sort();
    prereq_roles.iter().rev().for_each(|i| {
        roles.swap_remove(*i);
    });
    let mut role_list = RequirementList(roles);
    role_list.sort_alphabetical();
    Ok(role_list)
}

//TODO
// add in varibale lookup for differnt category roles/ascends
/// Check requirements for roles/ascendancies
#[poise::command(slash_command)]
pub async fn inn_items(
    ctx: Context<'_>,
    #[description = "User to lookup character of"] user: Option<User>,
    #[autocomplete = "autocomplete_character"]
    #[description = "character of selected user"]
    character: Option<i32>,
) -> Result<(), Error> {
    drop(user);
    let pool = &ctx.data().db_connection;
    let inn_list = get_requirements(INN_GUILD_ID,pool).await?;
    let items = if let Some(df_id) = character {
        let items = CharacterFetcher::new(df_id, LookupCategory::Ascendancies)
            .category(ParsingCategory::Items)
            .fetch_data()
            .await?
            .to_lookupstate()?
            .extract_character_data()?
            .item_list
            .take()
            .unwrap();
        Some(items)
    } else {
        None
    };
    let pages = get_requirement_pages(inn_list, items);
    let (r, g, b) = random_rgb();
    let embed = PaginateEmbed::new("Inn Items", None, Color::from_rgb(r, g, b), pages)
        .set_empty_string("No Inn Items to display");
    paginate(ctx, embed).await?;
    Ok(())
}
