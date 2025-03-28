use core::str;

use axum::{body::{Body, Bytes}, extract::{Multipart, State}, http::{header, Response, StatusCode}, response::IntoResponse, Json};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::{generate_response, storage::database, structs::{files::GetPDF, state::AppState}};

pub async fn submit_pdf(State(state): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {

    let mut data: Option<Bytes> = None;
    let mut file_name: Option<String> = None;
    let mut tag_values: Option<Vec<i32>> = None;

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

        if name.eq("tag_values") {
            let tag_bytes = match field.bytes().await {
                Ok(name_bytes) => name_bytes,
                Err(err) => {
                    return err.into_response();
                },
            };


            let tags_raw = match str::from_utf8(&tag_bytes) {
                Ok(tags_raw) => tags_raw.to_owned(),
                Err(err) => {
                    return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                },
            };

            let tags_split = tags_raw.split(' ');

            let mut tags = vec![];

            for tag in tags_split {
                tags.push(
                    match tag.parse::<i32>() {
                        Ok(id) => id,
                        Err(err) => {
                            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse tag_ids: {:?}", err.to_string()));
                        },
                    }
                );
            }

            tag_values = Some(tags);

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


    let mut file = match File::create(format!("files/{}.pdf", id)).await {
        Ok(file) => file,
        Err(err) => {
            return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
        },
    };

    let _ = file.write_all(&data.expect("Failed to fetch validated files")).await;

    if tag_values.is_some() {

        for tag_id in tag_values.expect("Failed to fetch validated List") {
            let _ = match database::add_file_tag(&state.database, &id, tag_id).await {
                Ok(_) => {},
                Err(err) => {
                    return generate_response!(StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
                }
            };
        }

    }

    
    generate_response!(StatusCode::OK, "Success!")
}

pub async fn retrieve_pdf(Json(file_id): Json<GetPDF>) -> impl IntoResponse {
    let file = match File::open(format!("files/{}.pdf", &file_id.uuid)).await {
        Ok(file) => file,
        Err(err) => {
            return generate_response!(StatusCode::NOT_FOUND, "No such File");
        },
    };

    let stream = ReaderStream::new(file);

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"display.pdf\"")
        .body(Body::from_stream(stream))
        .expect("Failed to generate Body from Stream!")
}















