use color_eyre::Result;
use getset::Getters;
use serde_derive::Deserialize;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::collections::{BTreeSet,HashSet};
use crate::parsing::{WarList,DFCharacterData};
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug, Deserialize,Eq,Hash,PartialEq)]
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

#[derive(Deserialize, Debug)]
pub struct RoleList(Vec<Role>);

impl RoleList {
    pub fn roles(&self) -> &Vec<Role> {
        &self.0
    }
    fn sort(&mut self) {
        self.0.sort_by(|a, b| max_last(&a, &b))
    }
}
#[derive(Debug, Deserialize, Eq, PartialEq,Hash)]
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
    ItemsAmount,
}

pub fn get_roles(path: &str) -> Result<RoleList> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut roles: RoleList = serde_json::from_reader(reader)?;
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
fn check_max_role(roles:&Vec<Role>,role: &Role, aquired_roles: &Vec<usize>) -> bool {
    let prereqs = role.prereqs().as_ref().expect("expected prereqs");
    let amount = prereqs.len();
    let mut has = 0;
    for index in aquired_roles{
        let role = roles.get(*index).as_ref().expect("expeced prereqs").name();
        if prereqs.contains(role){
            has += 1
        };
        if has >= amount{
            return true
        }
    };
    false
}
fn roles_to_remove<'a> (roles:&mut RoleList,mut char:DFCharacterData)->Vec<usize>{
    let char_items = char.item_list.take().unwrap().all();
    let roles_list = roles.roles();
    let mut roles_indexes_to_remove: Vec<usize>=vec![];
    for (i,role) in roles_list.iter().enumerate() {
        let aquired = match role.req_type {
            ReqType::Wars => check_war(role, &char),
            ReqType::Item => check_item(role, &char_items),
            ReqType::ItemsAmount => check_item_amount(role, &char_items),
            ReqType::Waves => check_waves(role, &char.wars),
            ReqType::Gold => check_gold(role, &char.gold()),
            ReqType::Max => check_max_role(roles_list,role, &roles_indexes_to_remove),
        };
        if aquired {
            roles_indexes_to_remove.push(i);
        }}
    roles_indexes_to_remove

}
pub fn check_roles(char: DFCharacterData) ->Result<RoleList>{
    let mut roles = get_roles("JSONS/roles.json")?;
    let mut to_remove = roles_to_remove(&mut roles, char);
    to_remove.sort_by(|a,b|b.cmp(a));
    let mut roles:Vec<Role> = to_remove.iter().map(|i|roles.0.remove(*i)).collect();
    to_remove.clear();
    for role in roles.iter(){
        if let Some(prereqs) = role.prereqs().as_ref(){
        for prereq in prereqs{
            for (i,checking_role) in roles.iter().enumerate(){
                if prereq == checking_role.name() && !to_remove.contains(&i){
                    to_remove.push(i);
                }

            }
        }
        }
    }
    to_remove.sort_by(|a,b|b.cmp(a));
    to_remove.iter().for_each(|i|{roles.remove(*i);});
    Ok(RoleList(roles))     
}
