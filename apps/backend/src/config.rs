use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry: i64,
    pub jwt_refresh_expiry: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_access_expiry: env::var("JWT_ACCESS_EXPIRY")?
                .parse()
                .expect("JWT_ACCESS_EXPIRY must be a number"),
            jwt_refresh_expiry: env::var("JWT_REFRESH_EXPIRY")?
                .parse()
                .expect("JWT_REFRESH_EXPIRY must be a number"),
        })
    }
}
