use anyhow::Error;
use axum::{body::Body, extract::Multipart, http::{Response, StatusCode}, response::IntoResponse};

pub async fn submit_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = match multipart.next_field().await {
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
        let data = match field.bytes().await {
            Ok(data) => data,
            Err(err) => {
                return err.into_response();
            },
        };
    }
    
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("meh"))
        .expect("Failed to statically generate response")

}
