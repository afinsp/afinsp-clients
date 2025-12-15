// this file is @generated
use serde::{Deserialize, Serialize};

use super::address::Address;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    pub same_as_billing: bool,
}

impl ShippingAddress {
    pub fn new(same_as_billing: bool) -> Self {
        Self {
            address: None,
            same_as_billing,
        }
    }
}
