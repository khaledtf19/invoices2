use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::ser;
use ts_rs::TS;

use crate::{
    error::ApiError,
    middleware::auth::AuthenticatedUser,
    models::user::{RefreshToken, User},
    response::ApiResponse,
    services::auth::AuthService,
    state::AppState,
};

#[derive(TS, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "Auth.ts", rename_all = "camelCase")]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "Auth.ts")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(TS, Debug, Deserialize)]
#[ts(export, export_to = "Auth.ts")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(TS, Debug, Serialize)]
#[ts(export, export_to = "Auth.ts")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
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

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    let auth_service = AuthService::new(
        state.config.jwt_secret.clone(),
        state.config.jwt_access_expiry,
        state.config.jwt_refresh_expiry,
    );

    if User::email_exists(&state.db, &payload.email).await? {
        return Err(ApiError::UserAlreadyExists);
    }

    if payload.password != payload.confirm_password {
        return Err(ApiError::InvalidCredentials);
    }

    let password_hash = auth_service.hash_password(&payload.password)?;

    let user = User::create(&state.db, &payload.email, &password_hash).await?;

    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_hash, expires_at) = auth_service.generate_refresh_token();

    RefreshToken::create(&state.db, user.id, &refresh_hash, expires_at).await?;

    Ok(ApiResponse::ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_expiry,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    let auth_service = AuthService::new(
        state.config.jwt_secret.clone(),
        state.config.jwt_access_expiry,
        state.config.jwt_refresh_expiry,
    );

    let user = User::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or(ApiError::InvalidCredentials)?;

    if !auth_service.verify_password(&payload.password, &user.password_hash)? {
        return Err(ApiError::InvalidCredentials);
    }

    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_hash, expires_at) = auth_service.generate_refresh_token();

    RefreshToken::create(&state.db, user.id, &refresh_hash, expires_at).await?;

    Ok(ApiResponse::ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_expiry,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
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

    Ok(ApiResponse::ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_expiry,
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, ApiError> {
    RefreshToken::delete_by_user_id(&state.db, auth_user.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth_user.user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?;

    Ok(ApiResponse::ok(UserResponse::from(user)))
}
