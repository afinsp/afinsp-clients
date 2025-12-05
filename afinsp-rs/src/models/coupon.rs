// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, coupon_id::CouponId, plan_id::PlanId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Coupon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,

    pub code: String,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub disabled: bool,

    pub discount: CouponDiscount,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    pub id: CouponId,

    pub plan_ids: Vec<PlanId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_value: Option<i32>,

    pub redemption_count: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_limit: Option<i32>,

    pub reusable: bool,
}

impl Coupon {
    pub fn new(
        code: String,
        created_at: String,
        disabled: bool,
        discount: CouponDiscount,
        id: CouponId,
        plan_ids: Vec<PlanId>,
        redemption_count: i32,
        reusable: bool,
    ) -> Self {
        Self {
            archived_at: None,
            code,
            created_at,
            description: None,
            disabled,
            discount,
            expires_at: None,
            id,
            plan_ids,
            recurring_value: None,
            redemption_count,
            redemption_limit: None,
            reusable,
        }
    }
}
