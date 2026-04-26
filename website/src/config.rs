use std::env;

pub struct Config {
    pub db_path: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> Self {
        Self {
            db_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "path/To/db.db".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3001),
        }
    }
}
