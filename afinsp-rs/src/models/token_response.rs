// this file is @generated
use serde::{Deserialize, Serialize};

/// Token response as per OAuth 2.0 spec
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,

    pub expires_in: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    pub token_type: String,
}

impl TokenResponse {
    pub fn new(access_token: String, expires_in: i32, token_type: String) -> Self {
        Self {
            access_token,
            expires_in,
            refresh_token: None,
            scope: None,
            token_type,
        }
    }
}
