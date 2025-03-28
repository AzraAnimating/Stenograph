use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    tag_value_ids: Option<Vec<i32>>
}
