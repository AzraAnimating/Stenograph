use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: i32, 
    value: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedTag {
    pub id: i32, 
    pub name: String,
    pub value_id: i32, 
    pub value: String
}
