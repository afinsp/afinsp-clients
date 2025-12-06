// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateOAuthAppRequest {
    pub name: String,

    pub redirect_uris: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl CreateOAuthAppRequest {
    pub fn new(name: String, redirect_uris: Vec<String>) -> Self {
        Self {
            name,
            redirect_uris,
            scopes: None,
        }
    }
}
