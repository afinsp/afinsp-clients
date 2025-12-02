// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    add_on_id::AddOnId, entitlement::Entitlement, price_id::PriceId,
    product_fee_type_enum::ProductFeeTypeEnum, product_id::ProductId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddOn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<ProductFeeTypeEnum>,

    pub id: AddOnId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances_per_subscription: Option<i32>,

    pub name: String,

    pub price_id: PriceId,

    pub product_id: ProductId,

    pub self_serviceable: bool,
}

impl AddOn {
    pub fn new(
        created_at: String,
        id: AddOnId,
        name: String,
        price_id: PriceId,
        product_id: ProductId,
        self_serviceable: bool,
    ) -> Self {
        Self {
            archived_at: None,
            created_at,
            description: None,
            entitlements: None,
            fee_type: None,
            id,
            max_instances_per_subscription: None,
            name,
            price_id,
            product_id,
            self_serviceable,
        }
    }
}
