// this file is @generated
use serde::{Deserialize, Serialize};

use super::{product_family_id::ProductFamilyId, product_fee_structure::ProductFeeStructure};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateProductRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub fee_structure: ProductFeeStructure,

    pub name: String,

    pub product_family_id: ProductFamilyId,
}

impl CreateProductRequest {
    pub fn new(
        fee_structure: ProductFeeStructure,
        name: String,
        product_family_id: ProductFamilyId,
    ) -> Self {
        Self {
            catalog: None,
            description: None,
            fee_structure,
            name,
            product_family_id,
        }
    }
}
