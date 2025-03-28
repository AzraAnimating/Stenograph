use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub server: ServerConfiguration
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfiguration {
    pub username: String, 
    pub password: String, 
    pub database: String,
    pub hostname: String, 
    pub port: u16,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfiguration {
    pub bind_addr: String
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration { 
            database: DatabaseConfiguration { username: "friedrich_wilhelm_xxvi".to_string(), password: "very_secure_password".to_string(), database: "protokolldb".to_string(), hostname: "localhost".to_string(), port: 5432 },
            server: ServerConfiguration { bind_addr: "0.0.0.0:8080".to_string() },
        }
    }
}
