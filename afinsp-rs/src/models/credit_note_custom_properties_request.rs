// this file is @generated
use serde::{Deserialize, Serialize};

/// Merge update of a credit note's custom property values (send a key with `null` to remove it).
/// Allowed at any status — custom properties stay editable after the credit note is finalized.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreditNoteCustomPropertiesRequest {
    pub custom_properties: serde_json::Value,
}

impl CreditNoteCustomPropertiesRequest {
    pub fn new(custom_properties: serde_json::Value) -> Self {
        Self { custom_properties }
    }
}
