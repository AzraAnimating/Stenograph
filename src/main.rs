use std::{fs, sync::Arc};

use axum::{routing::get, Router};
use deadpool_postgres::{Config, ManagerConfig};
use structs::configuration::Configuration;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;

mod structs;

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


    let mut database_config = Config::new(); 
    database_config.dbname = Some(config.database.database.clone());
    database_config.user = Some(config.database.username.clone()); 
    database_config.password = Some(config.database.password.clone());
    database_config.host = Some(config.database.hostname.clone()); 
    database_config.port = Some(config.database.port);
    database_config.manager = Some(ManagerConfig  {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast
    });
    let pool = database_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).expect("Failed to open Pool");


    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
    ;

    let listener = TcpListener::bind("0.0.0.0:8080").await.expect("Failed to bind!");
    axum::serve(listener, app).await.expect("Failed to start Server!");
}
