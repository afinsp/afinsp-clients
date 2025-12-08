// this file is @generated
use serde::{Deserialize, Serialize};

use super::{feature::Feature, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FeatureListResponse {
    pub data: Vec<Feature>,

    pub pagination_meta: PaginationResponse,
}

impl FeatureListResponse {
    pub fn new(data: Vec<Feature>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
