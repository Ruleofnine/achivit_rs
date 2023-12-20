use crate::parsing::{DFCharacterData, Items, WarList};
use std::collections::HashMap;
use color_eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
pub enum RolesListType {
    Roles,
    Ascend,
}
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub prereqs: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub req_type: ReqType,
    pub amount: Option<i32>,
}
impl Role {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn prereqs(&self) -> &Vec<String> {
        &self
            .prereqs
            .as_ref()
            .expect(format!("Role: {} Expected 'prereqs'", self.name()).as_str())
    }
    pub fn required(&self) -> &Vec<String> {
        &self
            .required
            .as_ref()
            .expect(format!("Role: {} Expected 'required'", self.name()).as_str())
    }
    pub fn amount(&self) -> u16 {
        self.amount
            .expect(format!("Role: {} Expected 'required'", self.name()).as_str()) as u16
    }
}
fn max_last(a: &Role, b: &Role) -> Ordering {
    match (&a.req_type, &b.req_type) {
        (ReqType::Max, ReqType::Max) => Ordering::Equal,
        (ReqType::Max, _) => Ordering::Greater,
        (_, ReqType::Max) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RoleList(Vec<Role>);

impl RoleList {
    pub fn roles(&self) -> &Vec<Role> {
        &self.0
    }
    fn sort(&mut self) {
        self.0.sort_by(|a, b| max_last(&a, &b))
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
    Inn
}
#[derive(Debug, Deserialize, Serialize)]
pub struct InnList{
    #[serde(flatten)]
    list:HashMap<String,InnReq>
}
impl InnList{
    pub fn reqs(&self)->&HashMap<String,InnReq>{
        &self.list
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InnReq{
    required:Vec<String>
}
impl InnReq{
    pub fn reqs(&self)->&Vec<String>{
        &self.required
    }
}


pub fn get_InnList() -> Result<InnList> {
    let file = File::open("JSONS/InnList.json")?;
    let reader = BufReader::new(file);
    let innlist: InnList = serde_json::from_reader(reader)?;
    Ok(innlist)
}
pub fn get_roles(path: &str) -> Result<RoleList> {
    let file = File::open(format!("JSONS/{path}"))?;
    let reader = BufReader::new(file);
    let mut roles: RoleList = serde_json::from_reader(reader)?;
    roles.sort();
    Ok(roles)
}

pub fn get_roles_bytes(bytes: &Vec<u8>) -> Result<RoleList> {
    let mut roles: RoleList = serde_json::from_slice(bytes)?;
    roles.sort();
    Ok(roles)
}
fn check_item(role: &Role, char_items: &Items) -> bool {
    let items = role
        .required
        .as_ref()
        .expect("Item Role requires item list");
    items.iter().all(|i| char_items.contains(i))
}
fn check_war(role: &Role, char: &DFCharacterData) -> bool {
    let amount = role.amount.expect("War needs amount") as usize;
    char.wars.wars().len() >= amount
}
fn check_item_amount(role: &Role, char_items: &Items) -> bool {
    let amount = role.amount();
    let items = role
        .required
        .as_ref()
        .expect("Item Role requires item list");
    let count = items.iter().filter(|&i| char_items.contains(i)).count();
    count as u16 >= amount
}

fn check_waves(role: &Role, wars: &WarList) -> bool {
    let amount = role.amount();
    wars.war_list()
        .iter()
        .any(|w| w.waves_int() as u16 >= amount)
}
fn check_gold(role: &Role, gold: &i32) -> bool {
    let amount = role.amount();
    *gold as u16 >= amount
}
fn check_max_role(roles: &Vec<Role>, role: &Role, aquired_roles: &Vec<usize>) -> bool {
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
fn check_item_stackable(role: &Role, items: &Items) -> bool {
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
                .trim_end_matches(")")
                .parse()
                .expect("failed to parse stackable amount");
            (name, amount)
        })
        .collect();
    required.iter().all(|(req_item, req_amount)| {
        items
            .items()
            .iter()
            .any(|(name,item)| name == req_item && item.amount() >= req_amount)
    })
}
fn aquired_roles_indexes<'a>(roles: &mut RoleList, mut char: DFCharacterData) -> Vec<usize> {
    let char_items = char.item_list.take().expect("expected char items");
    let dups = char_items.dups();
    let roles_list = roles.roles();
    let mut roles_indexes_to_remove: Vec<usize> = vec![];
    for (i, role) in roles_list.iter().enumerate() {
        let aquired = match role.req_type {
            ReqType::Wars => check_war(role, &char),
            ReqType::Item => check_item(role, &char_items),
            ReqType::ItemAmount => check_item_amount(role, &char_items),
            ReqType::Waves => check_waves(role, &char.wars),
            ReqType::Gold => check_gold(role, &char.gold()),
            ReqType::Max => check_max_role(roles_list, role, &roles_indexes_to_remove),
            ReqType::ItemUnique => role.amount() as u16 <= *char.unique_item_count(),
            ReqType::ItemLean => role.amount() as u16 <= *char.unique_item_count() && !dups,
            ReqType::ItemDC => role.amount() as u16 <= *char.dc_count(),
            ReqType::ItemStackable => check_item_stackable(role, &char_items),
            ReqType::Inn => check_max_role(roles_list,role, &roles_indexes_to_remove),
        };
        if aquired {
            roles_indexes_to_remove.push(i);
        }
    }
    roles_indexes_to_remove
}
fn prereq_roles_to_remove(roles: &Vec<Role>) -> Vec<usize> {
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
pub fn check_roles(char: DFCharacterData, path: &str) -> Result<RoleList> {
    let mut roles = get_roles(path)?;
    let mut aquired_roles = aquired_roles_indexes(&mut roles, char);
    aquired_roles.sort_by(|a, b| b.cmp(a));
    let mut roles: Vec<Role> = aquired_roles
        .iter()
        .map(|i| roles.0.swap_remove(*i))
        .collect();
    aquired_roles.sort_by(|a, b| b.cmp(a));
    let mut prereq_roles = prereq_roles_to_remove(&roles);
    prereq_roles.sort();
    prereq_roles.iter().rev().for_each(|i| {
        roles.swap_remove(*i);
    });
    let mut role_list = RoleList(roles);
    role_list.sort_alphabetical();
    Ok(role_list)
}
