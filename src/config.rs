use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost/hello_rust".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
