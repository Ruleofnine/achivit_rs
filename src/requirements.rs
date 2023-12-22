use crate::parsing::{DFCharacterData, Items, WarList};
use color_eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
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

pub fn get_requirements(path: &str) -> Result<RequirementList> {
    let file = File::open(format!("JSONS/{path}"))?;
    let reader = BufReader::new(file);
    let mut roles: RequirementList = serde_json::from_reader(reader)?;
    roles.sort();
    Ok(roles)
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
    let list = get_requirements("InnList.json").expect("failed to get in list");
    list.requirements()
        .iter()
        .all(|innreq| innreq.required().iter().all(|i| {dbg!(i);dbg!(items.contains(i))}))
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
pub fn check_requirements(char: &DFCharacterData, path: &str) -> Result<RequirementList> {
    let mut roles = get_requirements(path)?;
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
