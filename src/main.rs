use std::{fs, sync::Arc};

use axum::{routing::get, Router};
use deadpool_postgres::{Config, ManagerConfig};
use structs::configuration::Configuration;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;

mod structs;
mod controller;
mod database;
mod schema;
mod models;

#[tokio::main]
async fn main() {

    let config_str = match fs::read_to_string("config.toml") { //Load Config into String
        Ok(file) => file, 
        Err(err) => { //This way, we can easily check if it is there
            println!("Please populate the config.toml!");
            let config_default = toml::to_string_pretty(&Configuration::default()).expect("Failed to Serialize Default Config");
            fs::write("config.toml", config_default).expect("Failed to write default config!");
            return; //Or just write a Default one to Disk
        }
    };

    let config = Arc::new(toml::from_str::<Configuration>(&config_str).expect("Failed to parse Config!"));

    let database_connection = database::data::connect(config.database.clone());

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
    ;

    let listener = TcpListener::bind("0.0.0.0:8080").await.expect("Failed to bind!");
    axum::serve(listener, app).await.expect("Failed to start Server!");
}
