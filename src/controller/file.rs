use core::str;
use std::{fs::File, io::Write};

use axum::{body::{Body, Bytes}, extract::{Multipart, State}, http::{Response, StatusCode}, response::IntoResponse};

use crate::{generate_response, storage::database, structs::state::AppState};

pub async fn submit_pdf(State(state): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {

    let mut data: Option<Bytes> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(err) => {
            return err.into_response();
        },
    } {

        let name = match field.name() {
            Some(name) => name,
            None => {
                break;
            },
        };

        if name.eq("name") {

            let name_bytes = match field.bytes().await {
                Ok(name_bytes) => name_bytes,
                Err(err) => {
                    return err.into_response();
                },
            };


            file_name = match str::from_utf8(&name_bytes) {
                Ok(name) => Some(name.to_owned()),
                Err(err) => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR) 
                        .body(Body::from(err.to_string()))
                        .expect("Failed to statically generate response");
                },
            };
            continue;
        }

        if !name.eq("upload") {
            continue;
        }

        data = match field.bytes().await {
            Ok(data) => Some(data),
            Err(err) => {
                return err.into_response();
            },
        };

    }

    if data.is_none() || file_name.is_none() {
        return generate_response!(StatusCode::BAD_REQUEST, "Missing Parameter name or upload!");
    }

    let id = match database::add_file(&state.database, file_name.expect("Failed to get validated Value"), 1).await {
        Ok(id) => id,
        Err(err) => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };



    let mut file = match File::create(format!("files/{}.pdf", id)) {
        Ok(file) => file,
        Err(err) => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
        },
    };

    let _ = file.write(&data.expect("Failed to fetch validated files"));
    
    generate_response!(StatusCode::OK, "Success!")
}
