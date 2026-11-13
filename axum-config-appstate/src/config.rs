use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub jwt_secret: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub app: AppConfig,
    pub db: DbConfig,
}

impl Settings {
    pub fn new() -> Result<Self, envy::Error> {
        Ok(Self {
            app: envy::prefixed("APP_").from_env()?,
            db: envy::prefixed("DB_").from_env()?,
        })
    }
}
