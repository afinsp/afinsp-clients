// this file is @generated
use serde::{Deserialize, Serialize};

use super::{connection_type::ConnectionType, customer_id::CustomerId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateConnectedAccountRequest {
    pub connected_organization_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_customer_id: Option<CustomerId>,
}

impl CreateConnectedAccountRequest {
    pub fn new(connected_organization_id: String) -> Self {
        Self {
            connected_organization_id,
            connection_type: None,
            metadata: None,
            platform_customer_id: None,
        }
    }
}
