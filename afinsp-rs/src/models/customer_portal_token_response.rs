// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerPortalTokenResponse {
    /// Base URL of the customer portal
    pub portal_url: String,

    /// JWT token for portal access
    pub token: String,
}

impl CustomerPortalTokenResponse {
    pub fn new(portal_url: String, token: String) -> Self {
        Self { portal_url, token }
    }
}
