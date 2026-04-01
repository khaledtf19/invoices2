use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::ApiError,
    middleware::auth::AuthenticatedUser,
    models::user::{RefreshToken, User},
    services::auth::AuthService,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let auth_service = AuthService::new(
        state.config.jwt_secret.clone(),
        state.config.jwt_access_expiry,
        state.config.jwt_refresh_expiry,
    );

    if User::email_exists(&state.db, &payload.email).await? {
        return Err(ApiError::UserAlreadyExists);
    }

    let password_hash = auth_service.hash_password(&payload.password)?;

    let user = User::create(
        &state.db,
        &payload.email,
        &payload.first_name,
        &payload.last_name,
        &password_hash,
    )
    .await?;

    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_hash, expires_at) = auth_service.generate_refresh_token();

    RefreshToken::create(&state.db, user.id, &refresh_hash, expires_at).await?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_expiry,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
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

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_expiry,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
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

    Ok(Json(AuthResponse {
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
) -> Result<Json<UserResponse>, ApiError> {
    let user = User::find_by_id(&state.db, auth_user.user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?;

    Ok(Json(UserResponse::from(user)))
}
