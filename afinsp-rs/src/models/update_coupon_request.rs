// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, plan_id::PlanId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateCouponRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<CouponDiscount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ids: Option<Vec<PlanId>>,
}

impl UpdateCouponRequest {
    pub fn new() -> Self {
        Self {
            description: None,
            discount: None,
            plan_ids: None,
        }
    }
}
