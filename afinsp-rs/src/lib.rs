//! Afinsp Billing SDK for Rust.
//!
//! The main entry point of this library is the API client [`api::Afinsp`].
//!
//! # Example
//!
//! ```no_run
//! use afinsp_rs::api::{Afinsp, AfinspOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), afinsp_rs::error::Error> {
//!     let client = Afinsp::new("your-api-key".to_string(), None);
//!
//!     // List customers
//!     let customers = client.customers().list_customers(None).await?;
//!     println!("Found {} customers", customers.data.len());
//!
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]

use std::time::Duration;

use hyper::body::Bytes;
use hyper_util::client::legacy::Client as HyperClient;

pub mod api;
mod connector;
pub mod error;
pub mod models;
mod request;
pub mod webhooks;

pub(crate) use connector::{make_connector, Connector};

pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub bearer_access_token: Option<String>,
    pub timeout: Option<Duration>,
    pub num_retries: u32,
    pub retry_schedule: Option<Vec<Duration>>,

    client: HyperClient<Connector, http_body_util::Full<Bytes>>,
}
