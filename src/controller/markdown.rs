use axum::Json;

use crate::structs::markdown::MarkdownDocument;

pub fn submit_markdown_document(Json(markdown): Json<MarkdownDocument>) {
    
}
