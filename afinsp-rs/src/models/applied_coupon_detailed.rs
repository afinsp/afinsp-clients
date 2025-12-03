// this file is @generated
use serde::{Deserialize, Serialize};

use super::{applied_coupon::AppliedCoupon, subscription_coupon::SubscriptionCoupon};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AppliedCouponDetailed {
    pub applied_coupon: AppliedCoupon,

    pub coupon: SubscriptionCoupon,
}

impl AppliedCouponDetailed {
    pub fn new(applied_coupon: AppliedCoupon, coupon: SubscriptionCoupon) -> Self {
        Self {
            applied_coupon,
            coupon,
        }
    }
}
