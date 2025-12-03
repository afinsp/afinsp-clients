// this file is @generated
use serde::{Deserialize, Serialize};

/// Resets each time your subscription renews — anchored to your billing cycle.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BillingCycleResetPeriod {}

impl BillingCycleResetPeriod {
    pub fn new() -> Self {
        Self {}
    }
}
