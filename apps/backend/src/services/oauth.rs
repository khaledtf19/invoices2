use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};

use crate::error::ApiError;

// This type alias just saves us from writing the full generic type every time
pub type GoogleOAuthClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub fn build_oauth_client(
    client_id: String,
    client_secret: String,
    redirect_url: String,
) -> Result<GoogleOAuthClient, ApiError> {
    Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .map_err(|_| ApiError::InvalidCredentials)?,
        )
        .set_token_uri(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|_| ApiError::InvalidCredentials)?,
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url).map_err(|_| ApiError::InvalidCredentials)?,
        ))
}
