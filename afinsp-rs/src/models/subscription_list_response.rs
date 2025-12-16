// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pagination_response::PaginationResponse, subscription::Subscription};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionListResponse {
    pub data: Vec<Subscription>,

    pub pagination_meta: PaginationResponse,
}

impl SubscriptionListResponse {
    pub fn new(data: Vec<Subscription>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
