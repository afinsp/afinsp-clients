// this file is @generated
use serde::{Deserialize, Serialize};

/// Token introspection response as per RFC 7662
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TokenIntrospectionResponse {
    pub active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

impl TokenIntrospectionResponse {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            client_id: None,
            exp: None,
            iat: None,
            scope: None,
            sub: None,
            token_type: None,
        }
    }
}
