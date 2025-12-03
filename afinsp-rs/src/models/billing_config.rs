// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BillingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start_day: Option<i32>,
}

impl BillingConfig {
    pub fn new() -> Self {
        Self {
            billing_cycles: None,
            net_terms: None,
            period_start_day: None,
        }
    }
}
