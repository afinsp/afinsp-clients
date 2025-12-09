// this file is @generated
use serde::{Deserialize, Serialize};

/// Token introspection request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntrospectionRequest {
    /// The token to introspect
    pub token: String,
}

impl IntrospectionRequest {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
