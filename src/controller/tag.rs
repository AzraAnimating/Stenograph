use axum::{body::Body, extract::State, http::{Request, StatusCode}, Json};

use crate::{storage::database, structs::{state::AppState, tag::NamedTag}};

pub async fn get_tags(State(state): State<AppState>) ->Result<Json<Vec<NamedTag>>, StatusCode> {

    let tags = match database::get_all_tags(state.database.clone()).await {
        Ok(tags) => tags,
        Err(err) => {
            println!("Failed to fetch Tags!: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    Ok(Json(tags))
}

