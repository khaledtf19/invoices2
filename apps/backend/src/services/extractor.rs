use crate::{
    error::ApiError,
    services::auth::{AccessTokenClaims, AuthService},
    state::AppState,
};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;

impl FromRequestParts<AppState> for AccessTokenClaims {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let bearer_token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(str::to_owned);

        let cookie_token = CookieJar::from_headers(&parts.headers)
            .get("access_token")
            .map(|cookie| cookie.value().to_owned());

        let token = bearer_token.or(cookie_token).ok_or(ApiError::Unauthorized)?;

        let auth_service = AuthService::new(
            state.config.jwt_secret.clone(),
            state.config.jwt_access_expiry,
            state.config.jwt_refresh_expiry,
        );

        auth_service.verify_access_token(&token)
    }
}
