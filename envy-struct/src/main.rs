use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    database_url: String,
    port: u16,
    debug_mode: bool,
    #[serde(default = "default_timeout")]
    timeout_sec: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Deserialize, Debug)]
struct AppConfig {
    port: u16,
    env: String,
}

#[derive(Deserialize, Debug)]
struct DbConfig {
    host: String,
    port: u16,
    name: String,
}

#[derive(Deserialize, Debug)]
struct RedisConfig {
    url: String,
    pool_size: usize,
}

#[derive(Debug)]
struct Settings {
    app: AppConfig,
    db: DbConfig,
    redis: RedisConfig,
}

impl Settings {
    pub fn new() -> Result<Self, envy::Error> {
        let settings = Self {
            app: envy::prefixed("APP_").from_env()?,
            db: envy::prefixed("DB_").from_env()?,
            redis: envy::prefixed("REDIS_").from_env()?,
        };
        Ok(settings)
    }
}

fn main() {
    dotenvy::dotenv().ok();

    let config =
        envy::from_env::<Config>().expect("โหลด Config ไม่สำเร็จ กรุณาตรวจสอบ Environment Variables");
    println!("Loaded Config: {:#?}", config);
    println!("เชื่อมต่อ DB ที่: {}", config.database_url);

    let settings = Settings::new().expect("ไม่สามารถโหลด Settings ได้");
    println!("App Port: {}", settings.app.port);
    println!("DB Host: {}", settings.db.host);
    println!("Redis Pool: {}", settings.redis.pool_size);
}
