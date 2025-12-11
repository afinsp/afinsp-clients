// this file is @generated
use serde::{Deserialize, Serialize};

use super::o_auth_app::OAuthApp;

/// Result of creating an OAuth app (includes the plain-text secret)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OAuthAppWithSecret {
    pub app: OAuthApp,

    pub client_secret: String,
}

impl OAuthAppWithSecret {
    pub fn new(app: OAuthApp, client_secret: String) -> Self {
        Self { app, client_secret }
    }
}
