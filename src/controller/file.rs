use std::{fs::File, io::Write};

use axum::{body::{Body, HttpBody}, extract::{Multipart, State}, http::{Response, StatusCode}, response::IntoResponse};

use crate::structs::state::AppState;

pub async fn submit_pdf(State(state): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
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

        if !name.eq("upload.pdf") {
            continue;
        }

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(err) => {
                return err.into_response();
            },
        };

        let mut file = match File::create("upload.pdf") {
            Ok(file) => file,
            Err(err) => {
                println!("{:?}", err); 
                return Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from("meh"))
                    .expect("Failed to statically generate response");
            },
        };

        let _ = file.write(&data);
    }
    
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("meh"))
        .expect("Failed to statically generate response")

}
