// this file is @generated
use serde::{Deserialize, Serialize};

use super::price_entry::PriceEntry;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionAddOnPriceOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub price_entry: PriceEntry,
}

impl SubscriptionAddOnPriceOverride {
    pub fn new(price_entry: PriceEntry) -> Self {
        Self {
            name: None,
            price_entry,
        }
    }
}
