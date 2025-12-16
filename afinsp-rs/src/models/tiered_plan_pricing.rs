// this file is @generated
use serde::{Deserialize, Serialize};

use super::tier_row::TierRow;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TieredPlanPricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,

    pub tiers: Vec<TierRow>,
}

impl TieredPlanPricing {
    pub fn new(tiers: Vec<TierRow>) -> Self {
        Self {
            block_size: None,
            tiers,
        }
    }
}
