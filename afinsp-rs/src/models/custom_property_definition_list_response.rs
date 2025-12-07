// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    custom_property_definition::CustomPropertyDefinition, pagination_response::PaginationResponse,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomPropertyDefinitionListResponse {
    pub data: Vec<CustomPropertyDefinition>,

    pub pagination_meta: PaginationResponse,
}

impl CustomPropertyDefinitionListResponse {
    pub fn new(data: Vec<CustomPropertyDefinition>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
