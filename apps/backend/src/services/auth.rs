use chrono::{Duration, Utc};
use digest::Digest;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use rand::{RngCore, rngs::OsRng};
use sha2::Sha256;
use uuid::Uuid;

type Hs256 = Hmac<Sha256>;

use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_secret: String,
    access_expiry_secs: i64,
    refresh_expiry_days: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub email: String,
    pub user_id: Uuid,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub jti: String,
    pub exp: usize,
    pub iat: usize,
}

impl AuthService {
    pub fn new(jwt_secret: String, access_expiry_secs: i64, refresh_expiry_days: i64) -> Self {
        Self {
            jwt_secret,
            access_expiry_secs,
            refresh_expiry_days,
        }
    }

    fn get_key(&self) -> Hs256 {
        Hmac::new_from_slice(self.jwt_secret.as_bytes()).expect("HMAC can take key of any size")
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> Result<String, ApiError> {
        let now = Utc::now();
        let expiry = now + Duration::seconds(self.access_expiry_secs);

        let claims = AccessTokenClaims {
            sub: email.to_string(),
            email: email.to_string(),
            user_id: user_id,
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let key = self.get_key();
        let token = Token::new(Header::default(), claims).sign_with_key(&key);

        match token {
            Ok(t) => Ok(t.as_str().to_string()),
            Err(_) => Err(ApiError::InternalError(anyhow::anyhow!(
                "Failed to sign token"
            ))),
        }
    }

    pub fn generate_refresh_token(&self) -> (String, String, chrono::DateTime<Utc>) {
        let mut random_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut random_bytes);
        let token = hex::encode(random_bytes);
        let token_hash = Self::hash_token(&token);
        let expiry = Utc::now() + Duration::days(self.refresh_expiry_days);

        (token.to_string(), token_hash, expiry)
    }

    pub fn verify_access_token(&self, token_str: &str) -> Result<AccessTokenClaims, ApiError> {
        let key = self.get_key();
        let token: Token<Header, AccessTokenClaims, _> = token_str
            .verify_with_key(&key)
            .map_err(|_| ApiError::InvalidToken)?;

        Ok(token.claims().clone())
    }

    pub fn verify_refresh_token_hash(token: &str, stored_hash: &str) -> bool {
        let token_hash = Self::hash_token(token);
        token_hash == stored_hash
    }

    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, token.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn decode_refresh_token(&self, token_str: &str) -> Result<RefreshTokenClaims, ApiError> {
        let key = self.get_key();
        let token: Token<Header, RefreshTokenClaims, _> = token_str
            .verify_with_key(&key)
            .map_err(|_| ApiError::InvalidToken)?;

        Ok(token.claims().clone())
    }
}
