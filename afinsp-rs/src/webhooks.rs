//! Webhook signature verification.
//!
//! This module provides webhook signature verification with support for both
//! Standard Webhooks headers (`webhook-*`) and Svix headers (`svix-*`).
//!
//! # Example
//!
//! ```no_run
//! use afinsp_rs::webhooks::Webhook;
//!
//! fn verify_webhook(payload: &[u8], headers: &http1::HeaderMap) -> Result<(), afinsp_rs::webhooks::WebhookError> {
//!     let secret = "whsec_your_webhook_secret";
//!     let wh = Webhook::new(secret)?;
//!     wh.verify(payload, headers)?;
//!     Ok(())
//! }
//! ```
//!
//! # Supported Headers
//!
//! The verification supports two header naming conventions:
//!
//! **Standard Webhooks** (preferred):
//! - `webhook-id`: Unique message identifier
//! - `webhook-signature`: HMAC-SHA256 signature (format: `v1,<base64-signature>`)
//! - `webhook-timestamp`: Unix timestamp
//!
//! **Svix** (for compatibility):
//! - `svix-id`: Unique message identifier
//! - `svix-signature`: HMAC-SHA256 signature
//! - `svix-timestamp`: Unix timestamp
//!
//! Standard Webhooks headers take precedence if both are present.

pub use standardwebhooks::WebhookError;

const SVIX_ID: &str = "svix-id";
const SVIX_SIGNATURE: &str = "svix-signature";
const SVIX_TIMESTAMP: &str = "svix-timestamp";

/// Webhook signature verifier.
///
/// Supports both Standard Webhooks (`webhook-*`) and Svix (`svix-*`) header formats.
pub struct Webhook {
    inner: standardwebhooks::Webhook,
}

impl Webhook {
    /// Create a new webhook verifier from a secret.
    ///
    /// The secret can optionally be prefixed with `whsec_`.
    pub fn new(secret: &str) -> Result<Self, WebhookError> {
        Ok(Self {
            inner: standardwebhooks::Webhook::new(secret)?,
        })
    }

    /// Create a new webhook verifier from raw secret bytes.
    pub fn from_bytes(secret: Vec<u8>) -> Result<Self, WebhookError> {
        Ok(Self {
            inner: standardwebhooks::Webhook::from_bytes(secret)?,
        })
    }

    /// Verify a webhook payload and headers.
    ///
    /// This method enforces timestamp tolerance (5 minute window).
    pub fn verify(&self, payload: &[u8], headers: &http1::HeaderMap) -> Result<(), WebhookError> {
        let normalized = normalize_headers(headers);
        self.inner.verify(payload, &normalized)
    }

    /// Sign a payload and return the signature.
    ///
    /// Returns the signature in the format `v1,<base64-signature>`.
    pub fn sign(
        &self,
        msg_id: &str,
        timestamp: i64,
        payload: &[u8],
    ) -> Result<String, WebhookError> {
        self.inner.sign(msg_id, timestamp, payload)
    }
}

/// Normalizes headers by converting svix-* headers to webhook-* headers.
///
/// Standard webhook-* headers take precedence if both are present.
fn normalize_headers(headers: &http1::HeaderMap) -> http1::HeaderMap {
    let mut normalized = headers.clone();

    // For each svix header, copy to webhook header if webhook header is missing
    if !normalized.contains_key(standardwebhooks::HEADER_WEBHOOK_ID) {
        if let Some(value) = headers.get(SVIX_ID) {
            normalized.insert(
                http1::HeaderName::from_static(standardwebhooks::HEADER_WEBHOOK_ID),
                value.clone(),
            );
        }
    }

    if !normalized.contains_key(standardwebhooks::HEADER_WEBHOOK_SIGNATURE) {
        if let Some(value) = headers.get(SVIX_SIGNATURE) {
            normalized.insert(
                http1::HeaderName::from_static(standardwebhooks::HEADER_WEBHOOK_SIGNATURE),
                value.clone(),
            );
        }
    }

    if !normalized.contains_key(standardwebhooks::HEADER_WEBHOOK_TIMESTAMP) {
        if let Some(value) = headers.get(SVIX_TIMESTAMP) {
            normalized.insert(
                http1::HeaderName::from_static(standardwebhooks::HEADER_WEBHOOK_TIMESTAMP),
                value.clone(),
            );
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_headers(
        id_key: &str,
        sig_key: &str,
        ts_key: &str,
        signature: &str,
    ) -> http1::HeaderMap {
        let mut headers = http1::HeaderMap::new();
        headers.insert(
            http1::HeaderName::try_from(id_key).unwrap(),
            "msg_test123".parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::try_from(sig_key).unwrap(),
            signature.parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::try_from(ts_key).unwrap(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string()
                .parse()
                .unwrap(),
        );
        headers
    }

    #[test]
    fn test_verify_standard_headers() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD";
        let payload = br#"{"test": "data"}"#;
        let wh = Webhook::new(secret).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let signature = wh.sign("msg_test123", timestamp, payload).unwrap();
        let headers = make_headers(
            "webhook-id",
            "webhook-signature",
            "webhook-timestamp",
            &signature,
        );

        // Update timestamp in headers to match
        let mut headers = headers;
        headers.insert(
            http1::HeaderName::from_static("webhook-timestamp"),
            timestamp.to_string().parse().unwrap(),
        );

        wh.verify(payload, &headers).unwrap();
    }

    #[test]
    fn test_verify_svix_headers() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD";
        let payload = br#"{"test": "data"}"#;
        let wh = Webhook::new(secret).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let signature = wh.sign("msg_test123", timestamp, payload).unwrap();

        let mut headers = http1::HeaderMap::new();
        headers.insert(
            http1::HeaderName::from_static("svix-id"),
            "msg_test123".parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::from_static("svix-signature"),
            signature.parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::from_static("svix-timestamp"),
            timestamp.to_string().parse().unwrap(),
        );

        wh.verify(payload, &headers).unwrap();
    }

    #[test]
    fn test_invalid_signature() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD";
        let payload = br#"{"test": "data"}"#;
        let wh = Webhook::new(secret).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut headers = http1::HeaderMap::new();
        headers.insert(
            http1::HeaderName::from_static("webhook-id"),
            "msg_test123".parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::from_static("webhook-signature"),
            "v1,invalid_signature_here".parse().unwrap(),
        );
        headers.insert(
            http1::HeaderName::from_static("webhook-timestamp"),
            timestamp.to_string().parse().unwrap(),
        );

        assert!(wh.verify(payload, &headers).is_err());
    }
}
