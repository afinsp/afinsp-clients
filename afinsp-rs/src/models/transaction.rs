// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    customer_payment_method_id::CustomerPaymentMethodId, payment_method_info::PaymentMethodInfo,
    payment_status_enum::PaymentStatusEnum, payment_transaction_id::PaymentTransactionId,
    payment_type_enum::PaymentTypeEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub amount: i32,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    pub id: PaymentTransactionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<CustomerPaymentMethodId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_info: Option<PaymentMethodInfo>,

    pub payment_type: PaymentTypeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_transaction_id: Option<String>,

    pub status: PaymentStatusEnum,
}

impl Transaction {
    pub fn new(
        amount: i32,
        currency: String,
        id: PaymentTransactionId,
        payment_type: PaymentTypeEnum,
        status: PaymentStatusEnum,
    ) -> Self {
        Self {
            amount,
            currency,
            error: None,
            id,
            payment_method_id: None,
            payment_method_info: None,
            payment_type,
            processed_at: None,
            provider_transaction_id: None,
            status,
        }
    }
}
