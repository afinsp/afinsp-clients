// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    address::Address, currency::Currency, custom_tax_rate::CustomTaxRate,
    invoicing_entity_id::InvoicingEntityId, shipping_address::ShippingAddress,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerPatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Partial update of custom property values (merge; send a key with `null` to remove it).
    /// Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_taxes: Option<Vec<CustomTaxRate>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_emails: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_entity_id: Option<InvoicingEntityId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tax_exempt: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

impl CustomerPatchRequest {
    pub fn new() -> Self {
        Self {
            alias: None,
            billing_address: None,
            billing_email: None,
            currency: None,
            custom_properties: None,
            custom_taxes: None,
            invoicing_emails: None,
            invoicing_entity_id: None,
            is_tax_exempt: None,
            name: None,
            phone: None,
            shipping_address: None,
            vat_number: None,
        }
    }
}
