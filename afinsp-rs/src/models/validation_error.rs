//! Validation error type.

use serde::{Deserialize, Serialize};

/// A single validation error.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidationError {
    /// Location of the error (e.g., ["body", "field_name"]).
    pub loc: Vec<String>,
    /// Error message.
    pub msg: String,
    /// Error type identifier.
    #[serde(rename = "type")]
    pub error_type: String,
}
