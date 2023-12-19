use crate::parsing::{DFCharacterData, WarList};
use color_eyre::Result;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
pub enum RolesListType{
    Roles,
    Ascend
}
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Deserialize, Eq, Hash, PartialEq,Serialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub prereqs: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub req_type: ReqType,
    pub amount: Option<i32>,
}
fn max_last(a: &Role, b: &Role) -> Ordering {
    match (&a.req_type, &b.req_type) {
        (ReqType::Max, ReqType::Max) => Ordering::Equal,
        (ReqType::Max, _) => Ordering::Greater,
        (_, ReqType::Max) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

#[derive(Deserialize, Debug,Serialize)]
pub struct RoleList(Vec<Role>);

impl RoleList {
    pub fn roles(&self) -> &Vec<Role> {
        &self.0
    }
    fn sort(&mut self) {
        self.0.sort_by(|a, b| max_last(&a, &b))
    }
    pub fn sort_alphabetical(&mut self){
        self.0.sort_by(|a,b|a.name().cmp(b.name()))

    }
}
#[derive(Serialize,Debug, Deserialize, Eq, PartialEq, Hash)]
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
fn check_item(role: &Role, char_items: &BTreeSet<String>) -> bool {
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
fn check_item_amount(role: &Role, char_items: &BTreeSet<String>) -> bool {
    let amount = role.amount().expect("Items/Amount Needs amount.");
    let items = role
        .required
        .as_ref()
        .expect("Item Role requires item list");
    let count = items.iter().filter(|&i| char_items.contains(i)).count();
    count as i32 >= amount
}

fn check_waves(role: &Role, wars: &WarList) -> bool {
    let amount = role.amount().expect("Waves Needs amount.");
    wars.war_list().iter().any(|w| w.waves_int() >= amount)
}
fn check_gold(role: &Role, gold: &i32) -> bool {
    let amount = role.amount().expect("gold Needs amount.");
    gold >= &amount
}
fn check_max_role(roles: &Vec<Role>, role: &Role, aquired_roles: &Vec<usize>) -> bool {
    let prereqs = role.prereqs().as_ref().expect("expected prereqs");
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
fn aquired_roles_indexes<'a>(roles: &mut RoleList, mut char: DFCharacterData) -> Vec<usize> {
    let char_items = char.item_list.take().expect("expected char items");
    let dups = *char_items.dups();
    dbg!(dups);
    let char_items = char_items.all();
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
            ReqType::ItemUnique => role.amount().expect(&format!("{} needs amount",role.name())) as u16 <= *char.unique_item_count(),
            ReqType::ItemLean =>  role.amount().expect(&format!("{} needs amount",role.name())) as u16 <= *char.unique_item_count() && dups == 0,
            ReqType::ItemDC => role.amount().expect(&format!("{} needs amount",role.name())) as u16 <= *char.dc_count(),
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
        if let Some(prereqs) = role.prereqs().as_ref() {
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
pub fn check_roles(char: DFCharacterData,path:&str) -> Result<RoleList> {
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
