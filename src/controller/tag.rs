use axum::{body::Body, extract::State, http::{Request, StatusCode}, response::IntoResponse, Json};

use crate::{generate_response, storage::database, structs::{state::AppState, tag::{CreateTag, CreateTagValue, NamedValueTag}}};

pub async fn get_tags(State(state): State<AppState>) -> Result<Json<Vec<NamedValueTag>>, StatusCode> {

    let tags = match database::get_all_tags(state.database.clone()).await {
        Ok(tags) => tags,
        Err(err) => {
            println!("Failed to fetch Tags!: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    Ok(Json(tags))
}

pub async fn create_tag(State(state): State<AppState>, Json(create_tag): Json<CreateTag>) -> impl IntoResponse {
    
    let pool = &state.database;

    let tag_name = create_tag.name;

    let tag_id = match database::create_tag(&pool, tag_name).await {
        Ok(id) => id,
        Err(err) => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err)
        },
    };

    if create_tag.values.is_some() {
        let values = create_tag.values.expect("Failed to retrieve validated Value");


        for val in values {
            let _ = match database::create_tag_value(&pool, tag_id, &val.value).await {
                Ok(_) => {},
                Err(err) => {
                    return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err);
                },
            };

        }

    }

    return generate_response!(StatusCode::OK, "Success!")

}


pub async fn create_tag_value(State(state): State<AppState>, Json(tag_value): Json<CreateTagValue>) -> impl IntoResponse {

    let tag_id = match tag_value.tag_id {
        Some(id) => id,
        None => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, "Missing parameter tag_id");
        },
    };

    match database::create_tag_value(&state.database, tag_id, &tag_value.value).await {
        Ok(_) => {},
        Err(err) => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err)
        },
    }

    return generate_response!(StatusCode::OK, "Success!")
}
