// this file is @generated
use serde::{Deserialize, Serialize};

use super::bank_account_id::BankAccountId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BankTransferPaymentMethodConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<BankAccountId>,
}

impl BankTransferPaymentMethodConfig {
    pub fn new() -> Self {
        Self { account_id: None }
    }
}
