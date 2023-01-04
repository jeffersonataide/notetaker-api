use dotenvy::dotenv;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let db_url =
            std::env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
        DatabaseConfig { url: db_url }
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub port: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        ServerConfig { port }
    }
}

#[derive(Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            database: DatabaseConfig::new(),
            server: ServerConfig::new(),
        }
    }
}
