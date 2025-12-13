// this file is @generated
use serde::{Deserialize, Serialize};

use super::{existing_price_ref::ExistingPriceRef, price_input::PriceInput};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PriceEntry {
    #[serde(rename = "EXISTING")]
    Existing(ExistingPriceRef),
    #[serde(rename = "NEW")]
    New(PriceInput),
}

impl Default for PriceEntry {
    fn default() -> Self {
        Self::Existing(ExistingPriceRef::default())
    }
}
