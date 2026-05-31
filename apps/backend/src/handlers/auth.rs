use time::Duration;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{
    error::ApiError,
    models::{token::RefreshToken, user::User},
    response::ApiResponse,
    services::auth::{AccessTokenClaims, AuthService},
    state::AppState,
};

#[derive(TS, Debug, Deserialize)]
#[ts(export, export_to = "Auth.ts")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "Auth.ts")]
pub struct AuthResponse {
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "Auth.ts")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
        }
    }
}

pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<RefreshRequest>,
) -> Result<(CookieJar, ApiResponse<AuthResponse>), ApiError> {
    let auth_service = AuthService::new(
        state.config.jwt_secret.clone(),
        state.config.jwt_access_expiry,
        state.config.jwt_refresh_expiry,
    );

    let token_hash = AuthService::hash_token(&payload.refresh_token);

    let stored_token = RefreshToken::find_by_hash(&state.db, &token_hash)
        .await?
        .ok_or(ApiError::InvalidToken)?;

    if stored_token.expires_at < chrono::Utc::now() {
        RefreshToken::delete_by_hash(&state.db, &token_hash).await?;
        return Err(ApiError::TokenExpired);
    }

    if !AuthService::verify_refresh_token_hash(&payload.refresh_token, &stored_token.token_hash) {
        return Err(ApiError::InvalidToken);
    }

    let user = User::find_by_id(&state.db, stored_token.user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?;

    RefreshToken::delete_by_hash(&state.db, &token_hash).await?;

    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_hash, expires_at) = auth_service.generate_refresh_token();

    RefreshToken::create(&state.db, user.id, &refresh_hash, expires_at).await?;

    let cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::days(7))
        .build();

    let access_cookie = Cookie::build(("access_token", access_token))
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::seconds(900)) // 15min
        .build();

    Ok((
        jar.add(cookie).add(access_cookie),
        ApiResponse::ok(AuthResponse {
            token_type: "Bearer".to_string(),
            expires_in: state.config.jwt_access_expiry,
        }),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
) -> Result<impl IntoResponse, ApiError> {
    RefreshToken::delete_by_user_id(&state.db, claims.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = User::find_by_id(&state.db, claims.user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?;

    Ok(ApiResponse::ok(UserResponse::from(user)))
}
