// this file is @generated
use serde::{Deserialize, Serialize};

use super::price_id::PriceId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExistingPriceRef {
    pub id: PriceId,
}

impl ExistingPriceRef {
    pub fn new(id: PriceId) -> Self {
        Self { id }
    }
}
