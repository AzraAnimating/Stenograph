use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: i32, 
    name: i32
}

pub struct TagValue {
    tag_id: i32, 
    value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedTag {
    pub id: i32, 
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedValueTag {
    pub id: i32, 
    pub name: String,
    pub value_id: i32, 
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTag {
    pub name: String,
    pub values: Option<Vec<CreateTagValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTagValue {
    pub tag_id: Option<i32>,
    pub value: String,
}
