// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon::Coupon, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CouponListResponse {
    pub data: Vec<Coupon>,

    pub pagination_meta: PaginationResponse,
}

impl CouponListResponse {
    pub fn new(data: Vec<Coupon>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
