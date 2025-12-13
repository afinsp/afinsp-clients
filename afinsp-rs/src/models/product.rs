// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    product_family_id::ProductFamilyId, product_fee_structure::ProductFeeStructure,
    product_fee_type_enum::ProductFeeTypeEnum, product_id::ProductId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,

    pub catalog: bool,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub fee_structure: ProductFeeStructure,

    pub fee_type: ProductFeeTypeEnum,

    pub id: ProductId,

    pub name: String,

    pub product_family_id: ProductFamilyId,
}

impl Product {
    pub fn new(
        catalog: bool,
        created_at: String,
        fee_structure: ProductFeeStructure,
        fee_type: ProductFeeTypeEnum,
        id: ProductId,
        name: String,
        product_family_id: ProductFamilyId,
    ) -> Self {
        Self {
            archived_at: None,
            catalog,
            created_at,
            description: None,
            fee_structure,
            fee_type,
            id,
            name,
            product_family_id,
        }
    }
}
