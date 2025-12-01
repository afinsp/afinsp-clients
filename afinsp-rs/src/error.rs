//! Error types for the Afinsp SDK.

use std::fmt;

use http_body_util::BodyExt;
use hyper::body::{Bytes, Incoming};

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Afinsp API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Http Error
    Http(HttpErrorContent<crate::models::HttpErrorOut>),
    /// Http Validation Error
    Validation(HttpErrorContent<crate::models::HttpValidationError>),
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(format!("{err:?}"))
    }

    pub(crate) async fn from_response(status_code: http1::StatusCode, body: Incoming) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if status_code == http1::StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: status_code,
                        payload: serde_json::from_slice(&bytes).ok(),
                        raw_body: bytes,
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: status_code,
                        payload: serde_json::from_slice(&bytes).ok(),
                        raw_body: bytes,
                    })
                }
            }
            Err(e) => Self::Generic(e.to_string()),
        }
    }
}

impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Error::Http(e) => write!(
                f,
                "Http error (status={}) payload={:?} body={}",
                e.status,
                e.payload,
                e.body_as_str()
            ),
            Error::Validation(e) => write!(
                f,
                "Validation error payload={:?} body={}",
                e.payload,
                e.body_as_str()
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub struct HttpErrorContent<T> {
    pub status: http1::StatusCode,
    /// Parsed payload if the body matched the expected error schema. `None`
    /// when the server returned a body in a different shape; inspect
    /// [`Self::raw_body`] in that case.
    pub payload: Option<T>,
    /// Raw response body, always captured so debugging is possible even when
    /// the server's error format doesn't match the SDK's expected schema.
    pub raw_body: Bytes,
}

impl<T> HttpErrorContent<T> {
    /// Lossy UTF-8 view of the raw response body, for logging.
    pub fn body_as_str(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.raw_body)
    }
}

impl<T: fmt::Debug> fmt::Debug for HttpErrorContent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpErrorContent")
            .field("status", &self.status)
            .field("payload", &self.payload)
            .field("raw_body", &self.body_as_str())
            .finish()
    }
}
