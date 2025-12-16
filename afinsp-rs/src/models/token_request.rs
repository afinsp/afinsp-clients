// this file is @generated
use serde::{Deserialize, Serialize};

/// Token request (from POST body, application/x-www-form-urlencoded)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TokenRequest {
    /// Client ID (if not using HTTP Basic auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Client secret (if not using HTTP Basic auth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Authorization code (for authorization_code grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// PKCE code verifier (for authorization_code grant with PKCE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,

    /// Grant type: "authorization_code" or "refresh_token"
    pub grant_type: String,

    /// Redirect URI (for authorization_code grant, must match the one used in /authorize)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,

    /// Refresh token (for refresh_token grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl TokenRequest {
    pub fn new(grant_type: String) -> Self {
        Self {
            client_id: None,
            client_secret: None,
            code: None,
            code_verifier: None,
            grant_type,
            redirect_uri: None,
            refresh_token: None,
        }
    }
}
