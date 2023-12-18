use color_eyre::Result;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::BufReader;
use getset::Getters;
use std::cmp::Ordering;
#[derive(Getters)]
#[getset(get = "pub")]
#[derive(Debug,Deserialize)]
pub struct Role{
    pub name:String,
    pub description:String,
    pub prereqs:Option<Vec<String>>,
    pub required:Option<Vec<String>>,
    #[serde(rename = "type")]
    pub req_type:ReqType,
    pub amount:Option<i32>,

}
fn max_last(a:&Role,b:&Role)->Ordering{
            match (&a.req_type,&b.req_type){
                (ReqType::Max,ReqType::Max)=>Ordering::Equal,
                (ReqType::Max,_)=>Ordering::Greater,
                (_,ReqType::Max)=>Ordering::Less,
                _  =>Ordering::Equal


            }
}

#[derive(Deserialize, Debug)]
pub struct RoleList(Vec<Role>);

impl RoleList{
    pub fn roles(&self) -> &Vec<Role> {
        &self.0
    }
    fn sort(&mut self){
        self.0.sort_by(|a,b| max_last(&a,&b))}
}
#[derive(Debug,Deserialize,Eq,PartialEq)]
#[allow(non_snake_case)]
#[serde(rename_all = "PascalCase")]
pub enum ReqType{
    Wars,
    Waves,
    Item,
    Gold,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "Item/Amount")]
    ItemsAmount,
}

pub fn get_roles(path:&str)->Result<RoleList>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut roles: RoleList = serde_json::from_reader(reader)?;
    roles.sort();
    Ok(roles)
}
