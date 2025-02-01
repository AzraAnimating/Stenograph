use axum::{extract::{Request, State}, response::IntoResponse};

use crate::structs::state::AppState;

pub fn get_tags(request: Request, State(state): State<AppState>) {

}
