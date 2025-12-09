//! HTTP validation error response type.

use serde::{Deserialize, Serialize};

use super::ValidationError;

/// HTTP validation error response (422).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpValidationError {
    /// List of validation errors.
    pub detail: Vec<ValidationError>,
}
