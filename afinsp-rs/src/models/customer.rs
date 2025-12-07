// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    address::Address, currency::Currency, custom_tax_rate::CustomTaxRate, customer_id::CustomerId,
    invoicing_entity_id::InvoicingEntityId, shipping_address::ShippingAddress,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account_id: Option<String>,

    pub currency: Currency,

    /// User-defined custom property values, keyed by definition `key`.
    pub custom_properties: serde_json::Value,

    pub custom_taxes: Vec<CustomTaxRate>,

    pub id: CustomerId,

    pub invoicing_emails: Vec<String>,

    pub invoicing_entity_id: InvoicingEntityId,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
}

impl Customer {
    pub fn new(
        currency: Currency,
        custom_properties: serde_json::Value,
        custom_taxes: Vec<CustomTaxRate>,
        id: CustomerId,
        invoicing_emails: Vec<String>,
        invoicing_entity_id: InvoicingEntityId,
        name: String,
    ) -> Self {
        Self {
            alias: None,
            billing_address: None,
            billing_email: None,
            connected_account_id: None,
            currency,
            custom_properties,
            custom_taxes,
            id,
            invoicing_emails,
            invoicing_entity_id,
            name,
            phone: None,
            shipping_address: None,
            vat_number: None,
        }
    }
}
