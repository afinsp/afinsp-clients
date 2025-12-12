// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PaymentMethodTypeEnum {
    #[default]
    #[serde(rename = "CARD")]
    Card,

    #[serde(rename = "BANK_TRANSFER")]
    BankTransfer,

    #[serde(rename = "WALLET")]
    Wallet,

    #[serde(rename = "OTHER")]
    Other,
}

impl fmt::Display for PaymentMethodTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Card => "CARD",
            Self::BankTransfer => "BANK_TRANSFER",
            Self::Wallet => "WALLET",
            Self::Other => "OTHER",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for PaymentMethodTypeEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
