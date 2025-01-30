use serde::{Deserialize, Serialize};

use super::tag::Tag;

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownDocument {
    pub name: String, 
    pub content: String, 
    pub tags: Vec<Tag>
}
