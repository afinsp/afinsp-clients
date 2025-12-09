// this file is @generated
use serde::{Deserialize, Serialize};

use super::sub_line_item::SubLineItem;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub amount_total: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub end_date: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<rust_decimal::Decimal>,

    pub start_date: String,

    pub sub_line_items: Vec<SubLineItem>,

    pub tax_rate: rust_decimal::Decimal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<rust_decimal::Decimal>,
}

impl InvoiceLineItem {
    pub fn new(
        amount_total: i32,
        end_date: String,
        name: String,
        start_date: String,
        sub_line_items: Vec<SubLineItem>,
        tax_rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            amount_total,
            description: None,
            end_date,
            name,
            quantity: None,
            start_date,
            sub_line_items,
            tax_rate,
            unit_price: None,
        }
    }
}
