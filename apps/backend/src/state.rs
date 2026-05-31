use crate::config::Config;
use crate::services::oauth::GoogleOAuthClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub oauth_client: GoogleOAuthClient,
}

impl AppState {
    pub fn new(db: PgPool, config: Config, oauth_client: GoogleOAuthClient) -> Self {
        Self {
            db,
            config,
            oauth_client,
        }
    }
}
