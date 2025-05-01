use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GetFile {
    pub uuid: String
}

pub enum DataType {
    HTML = 0,
    PDF = 1, 
}
