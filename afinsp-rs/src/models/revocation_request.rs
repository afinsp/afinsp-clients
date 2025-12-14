// this file is @generated
use serde::{Deserialize, Serialize};

/// Token revocation request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevocationRequest {
    /// The token to revoke
    pub token: String,

    /// Optional hint about the token type (access_token or refresh_token)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<String>,
}

impl RevocationRequest {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type_hint: None,
        }
    }
}
