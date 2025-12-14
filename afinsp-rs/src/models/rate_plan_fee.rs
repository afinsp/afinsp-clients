// this file is @generated
use serde::{Deserialize, Serialize};

use super::term_rate::TermRate;

/// Recurring rate fee (e.g., monthly subscription)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RatePlanFee {
    pub rates: Vec<TermRate>,
}

impl RatePlanFee {
    pub fn new(rates: Vec<TermRate>) -> Self {
        Self { rates }
    }
}
