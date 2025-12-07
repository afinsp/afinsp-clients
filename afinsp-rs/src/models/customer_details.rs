// this file is @generated
use serde::{Deserialize, Serialize};

use super::{address::Address, customer_id::CustomerId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    pub id: CustomerId,

    pub name: String,

    pub snapshot_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

impl CustomerDetails {
    pub fn new(id: CustomerId, name: String, snapshot_at: String) -> Self {
        Self {
            alias: None,
            billing_address: None,
            email: None,
            id,
            name,
            snapshot_at,
            vat_number: None,
        }
    }
}
