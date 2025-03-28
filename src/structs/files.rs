use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GetPDF {
    pub uuid: String
}
