// this file is @generated
use serde::{Deserialize, Serialize};

use super::{applied_coupon_id::AppliedCouponId, coupon_id::CouponId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AppliedCoupon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_amount: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_count: Option<i32>,

    pub coupon_id: CouponId,

    pub created_at: String,

    pub id: AppliedCouponId,

    pub is_active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_applied_at: Option<String>,
}

impl AppliedCoupon {
    pub fn new(
        coupon_id: CouponId,
        created_at: String,
        id: AppliedCouponId,
        is_active: bool,
    ) -> Self {
        Self {
            applied_amount: None,
            applied_count: None,
            coupon_id,
            created_at,
            id,
            is_active,
            last_applied_at: None,
        }
    }
}
