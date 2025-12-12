// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    bank_transfer_payment_method_config::BankTransferPaymentMethodConfig,
    external_payment_method_config::ExternalPaymentMethodConfig,
    online_payment_method_config::OnlinePaymentMethodConfig,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PaymentMethodsConfig {
    #[serde(rename = "online")]
    Online(OnlinePaymentMethodConfig),
    #[serde(rename = "bank_transfer")]
    BankTransfer(BankTransferPaymentMethodConfig),
    #[serde(rename = "external")]
    External(ExternalPaymentMethodConfig),
}

impl Default for PaymentMethodsConfig {
    fn default() -> Self {
        Self::Online(OnlinePaymentMethodConfig::default())
    }
}
