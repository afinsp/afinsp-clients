//! HTTP error response type.

use serde::{Deserialize, Serialize};

/// HTTP error response from the API.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpErrorOut {
    /// Error code identifier.
    pub code: String,
    /// Human-readable error message.
    pub detail: String,
}
