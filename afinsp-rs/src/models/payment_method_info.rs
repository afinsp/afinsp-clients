// this file is @generated
use serde::{Deserialize, Serialize};

use super::payment_method_type_enum::PaymentMethodTypeEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentMethodInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number_hint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,

    pub payment_method_type: PaymentMethodTypeEnum,
}

impl PaymentMethodInfo {
    pub fn new(payment_method_type: PaymentMethodTypeEnum) -> Self {
        Self {
            account_number_hint: None,
            card_brand: None,
            card_last4: None,
            payment_method_type,
        }
    }
}
