// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    product_fee_structure::ProductFeeStructure, product_fee_type_enum::ProductFeeTypeEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NewProductRef {
    pub fee_structure: ProductFeeStructure,

    pub fee_type: ProductFeeTypeEnum,

    pub name: String,
}

impl NewProductRef {
    pub fn new(
        fee_structure: ProductFeeStructure,
        fee_type: ProductFeeTypeEnum,
        name: String,
    ) -> Self {
        Self {
            fee_structure,
            fee_type,
            name,
        }
    }
}
