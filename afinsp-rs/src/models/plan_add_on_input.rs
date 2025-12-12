// this file is @generated
use serde::{Deserialize, Serialize};

use super::{add_on_id::AddOnId, price_id::PriceId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanAddOnInput {
    pub add_on_id: AddOnId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_id: Option<PriceId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_serviceable: Option<bool>,
}

impl PlanAddOnInput {
    pub fn new(add_on_id: AddOnId) -> Self {
        Self {
            add_on_id,
            max_instances: None,
            price_id: None,
            self_serviceable: None,
        }
    }
}
