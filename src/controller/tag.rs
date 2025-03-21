use axum::{extract::State, http::StatusCode};

use crate::structs::state::AppState;

pub async fn get_tags(State(state): State<AppState>) ->Result<String, StatusCode> {
    
    Ok("".to_string())
}
