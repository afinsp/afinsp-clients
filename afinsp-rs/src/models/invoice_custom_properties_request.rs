// this file is @generated
use serde::{Deserialize, Serialize};

/// Merge update of an invoice's custom property values (send a key with `null` to remove it).
/// Allowed at any status — custom properties stay editable after the invoice is finalized.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InvoiceCustomPropertiesRequest {
    pub custom_properties: serde_json::Value,
}

impl InvoiceCustomPropertiesRequest {
    pub fn new(custom_properties: serde_json::Value) -> Self {
        Self { custom_properties }
    }
}
