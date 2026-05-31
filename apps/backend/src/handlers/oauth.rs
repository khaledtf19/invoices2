use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use time::Duration;

use crate::{
    error::ApiError,
    models::{
        oauth::{GoogleCallbackParams, GoogleUserProfile},
        token::RefreshToken,
        user::User,
    },
    services::auth::AuthService,
    state::AppState,
};

pub async fn google_auth(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), ApiError> {
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    let csrf_cookie = Cookie::build(("oauth_csrf", csrf_token.secret().clone()))
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::minutes(10))
        .build();

    Ok((jar.add(csrf_cookie), Redirect::to(auth_url.as_str())))
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(params): Query<GoogleCallbackParams>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), ApiError> {
    let stored_csrf = jar
        .get("oauth_csrf")
        .map(|c| c.value().to_string())
        .ok_or(ApiError::InvalidToken)?;

    if stored_csrf != params.state {
        return Err(ApiError::InvalidToken);
    }

    let jar = jar.remove(Cookie::from("oauth_csrf"));
    let oauth_http_client = oauth2::reqwest::Client::new();

    let token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&oauth_http_client)
        .await
        .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Token exchange failed: {e}")))?;

    let http_client = reqwest::Client::new();
    let profile = http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Failed to fetch profile: {e}")))?
        .json::<GoogleUserProfile>()
        .await
        .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Failed to parse profile: {e}")))?;

    if !profile.email_verified {
        return Err(ApiError::Unauthorized);
    }

    let user = match User::find_by_email(&state.db, &profile.email).await? {
        Some(existing) => existing,
        None => {
            User::create_oauth_user(
                &state.db,
                &profile.email,
                &profile.given_name,
                &profile.family_name,
            )
            .await?
        }
    };

    let auth_service = AuthService::new(
        state.config.jwt_secret.clone(),
        state.config.jwt_access_expiry,
        state.config.jwt_refresh_expiry,
    );

    let access_token = auth_service.generate_access_token(user.id, &user.email)?;
    let (refresh_token, refresh_hash, expires_at) = auth_service.generate_refresh_token();
    RefreshToken::create(&state.db, user.id, &refresh_hash, expires_at).await?;

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::days(state.config.jwt_refresh_expiry))
        .build();

    let access_cookie = Cookie::build(("access_token", access_token))
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::seconds(state.config.jwt_access_expiry))
        .build();

    let frontend_url = format!(
        "{}/auth/callback",
        state.config.frontend_url.trim_end_matches('/')
    );

    Ok((
        jar.add(refresh_cookie).add(access_cookie),
        Redirect::to(&frontend_url),
    ))
}
