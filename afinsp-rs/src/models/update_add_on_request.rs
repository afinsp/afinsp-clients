// this file is @generated
use serde::{Deserialize, Serialize};

use super::price_id::PriceId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAddOnRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances_per_subscription: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_id: Option<PriceId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_serviceable: Option<bool>,
}

impl UpdateAddOnRequest {
    pub fn new() -> Self {
        Self {
            description: None,
            max_instances_per_subscription: None,
            name: None,
            price_id: None,
            self_serviceable: None,
        }
    }
}
