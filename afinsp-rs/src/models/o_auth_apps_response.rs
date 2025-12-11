// this file is @generated
use serde::{Deserialize, Serialize};

use super::o_auth_app::OAuthApp;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OAuthAppsResponse {
    pub data: Vec<OAuthApp>,
}

impl OAuthAppsResponse {
    pub fn new(data: Vec<OAuthApp>) -> Self {
        Self { data }
    }
}
