// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    add_on_id::AddOnId, subscription_add_on_customization::SubscriptionAddOnCustomization,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateSubscriptionAddOn {
    pub add_on_id: AddOnId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization: Option<SubscriptionAddOnCustomization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl CreateSubscriptionAddOn {
    pub fn new(add_on_id: AddOnId) -> Self {
        Self {
            add_on_id,
            customization: None,
            quantity: None,
        }
    }
}
