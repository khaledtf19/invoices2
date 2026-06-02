use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleUserProfile {
    pub sub: String, // Google's unique user ID
    pub email: String,
    pub name: String,
    pub given_name: String,  // first name
    pub family_name: String, // last name
    pub picture: Option<String>,
    pub email_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackParams {
    pub code: String,  // exchange this for a token
    pub state: String, // this is the CSRF token to verify
}
