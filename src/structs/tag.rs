use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: i32, 
    value: i32
}
