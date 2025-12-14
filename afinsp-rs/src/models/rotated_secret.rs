// this file is @generated
use serde::{Deserialize, Serialize};

/// Result of rotating a client secret
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RotatedSecret {
    pub client_secret: String,

    pub client_secret_hint: String,
}

impl RotatedSecret {
    pub fn new(client_secret: String, client_secret_hint: String) -> Self {
        Self {
            client_secret,
            client_secret_hint,
        }
    }
}
