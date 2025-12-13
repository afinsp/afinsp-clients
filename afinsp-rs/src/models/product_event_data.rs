// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    product_family_id::ProductFamilyId, product_fee_type_enum::ProductFeeTypeEnum,
    product_id::ProductId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductEventData {
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub fee_type: ProductFeeTypeEnum,

    pub name: String,

    pub product_family_id: ProductFamilyId,

    pub product_id: ProductId,
}

impl ProductEventData {
    pub fn new(
        created_at: String,
        fee_type: ProductFeeTypeEnum,
        name: String,
        product_family_id: ProductFamilyId,
        product_id: ProductId,
    ) -> Self {
        Self {
            created_at,
            description: None,
            fee_type,
            name,
            product_family_id,
            product_id,
        }
    }
}
