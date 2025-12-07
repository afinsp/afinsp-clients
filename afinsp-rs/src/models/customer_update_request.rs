// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    address::Address, currency::Currency, custom_tax_rate::CustomTaxRate,
    invoicing_entity_id::InvoicingEntityId, shipping_address::ShippingAddress,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    pub currency: Currency,

    /// User-defined custom property values (full replace). Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    pub custom_taxes: Vec<CustomTaxRate>,

    pub invoicing_emails: Vec<String>,

    pub invoicing_entity_id: InvoicingEntityId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tax_exempt: Option<bool>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

impl CustomerUpdateRequest {
    pub fn new(
        currency: Currency,
        custom_taxes: Vec<CustomTaxRate>,
        invoicing_emails: Vec<String>,
        invoicing_entity_id: InvoicingEntityId,
        name: String,
    ) -> Self {
        Self {
            alias: None,
            billing_address: None,
            billing_email: None,
            currency,
            custom_properties: None,
            custom_taxes,
            invoicing_emails,
            invoicing_entity_id,
            is_tax_exempt: None,
            name,
            phone: None,
            shipping_address: None,
            vat_number: None,
        }
    }
}
