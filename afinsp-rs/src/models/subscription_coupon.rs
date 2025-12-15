// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_discount::CouponDiscount, coupon_id::CouponId};

/// Coupon as embedded in subscription details — a subset of the `Coupon` resource
/// returned by the coupons API.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionCoupon {
    pub code: String,

    pub description: String,

    pub disabled: bool,

    pub discount: CouponDiscount,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    pub id: CouponId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_value: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_limit: Option<i32>,

    pub reusable: bool,
}

impl SubscriptionCoupon {
    pub fn new(
        code: String,
        description: String,
        disabled: bool,
        discount: CouponDiscount,
        id: CouponId,
        reusable: bool,
    ) -> Self {
        Self {
            code,
            description,
            disabled,
            discount,
            expires_at: None,
            id,
            recurring_value: None,
            redemption_limit: None,
            reusable,
        }
    }
}
