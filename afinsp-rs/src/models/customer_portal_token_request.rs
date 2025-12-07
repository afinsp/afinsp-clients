// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerPortalTokenRequest {
    /// Token lifetime in seconds. Defaults to 86400 (24 hours).
    /// Must be between 60 and 2592000 (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<i32>,
}

impl CustomerPortalTokenRequest {
    pub fn new() -> Self {
        Self {
            expires_in_seconds: None,
        }
    }
}
