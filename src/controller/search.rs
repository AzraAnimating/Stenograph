use axum::{extract::State, response::IntoResponse, Json};

use crate::structs::{search::Search, state::AppState};

pub async fn search(State(state): State<AppState>, Json(search): Json<Search>) -> impl IntoResponse {
    
}
