use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry: i64,
    pub jwt_refresh_expiry: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub frontend_url: String,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let frontend_url = env::var("FRONTEND_URL")?;
        let cookie_secure = env::var("COOKIE_SECURE")
            .ok()
            .and_then(|value| value.parse::<bool>().ok())
            .unwrap_or_else(|| {
                !(frontend_url.starts_with("http://localhost") || frontend_url.starts_with("http://127.0.0.1"))
            });

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_access_expiry: env::var("JWT_ACCESS_EXPIRY")?
                .parse()
                .expect("JWT_ACCESS_EXPIRY must be a number"),
            jwt_refresh_expiry: env::var("JWT_REFRESH_EXPIRY")?
                .parse()
                .expect("JWT_REFRESH_EXPIRY must be a number"),
            google_client_id: env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")?,
            google_redirect_url: env::var("GOOGLE_REDIRECT_URL")?,
            frontend_url,
            cookie_secure,
        })
    }
}
