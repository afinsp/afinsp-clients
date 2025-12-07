// this file is @generated
use serde::{Deserialize, Serialize};

use super::customer_id::CustomerId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerEventData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,

    pub currency: String,

    /// User-defined custom property values, keyed by definition key.
    pub custom_properties: serde_json::Value,

    pub customer_id: CustomerId,

    pub invoicing_emails: Vec<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl CustomerEventData {
    pub fn new(
        currency: String,
        custom_properties: serde_json::Value,
        customer_id: CustomerId,
        invoicing_emails: Vec<String>,
        name: String,
    ) -> Self {
        Self {
            alias: None,
            billing_email: None,
            currency,
            custom_properties,
            customer_id,
            invoicing_emails,
            name,
            phone: None,
        }
    }
}
