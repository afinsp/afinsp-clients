// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, plan_id::PlanId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateCouponRequest {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub discount: CouponDiscount,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ids: Option<Vec<PlanId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_value: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
}

impl CreateCouponRequest {
    pub fn new(code: String, discount: CouponDiscount) -> Self {
        Self {
            code,
            description: None,
            discount,
            expires_at: None,
            plan_ids: None,
            recurring_value: None,
            redemption_limit: None,
            reusable: None,
        }
    }
}
