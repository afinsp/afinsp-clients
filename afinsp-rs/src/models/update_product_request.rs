// this file is @generated
use serde::{Deserialize, Serialize};

use super::product_fee_structure::ProductFeeStructure;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateProductRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_structure: Option<ProductFeeStructure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateProductRequest {
    pub fn new() -> Self {
        Self {
            description: None,
            fee_structure: None,
            name: None,
        }
    }
}
