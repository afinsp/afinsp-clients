// this file is @generated
use serde::{Deserialize, Serialize};

use super::tax_exemption_type::TaxExemptionType;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TaxBreakdownItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_type: Option<TaxExemptionType>,

    pub name: String,

    pub tax_amount: i32,

    pub tax_rate: rust_decimal::Decimal,

    pub taxable_amount: i32,
}

impl TaxBreakdownItem {
    pub fn new(
        name: String,
        tax_amount: i32,
        tax_rate: rust_decimal::Decimal,
        taxable_amount: i32,
    ) -> Self {
        Self {
            exemption_type: None,
            name,
            tax_amount,
            tax_rate,
            taxable_amount,
        }
    }
}
