// this file is @generated
//! Afinsp API client.

use std::{sync::Arc, time::Duration};

use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};

use crate::Configuration;

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Options for configuring the Afinsp client.
#[derive(Debug, Clone)]
pub struct AfinspOptions {
    /// Enable debug logging.
    pub debug: bool,

    /// Custom server URL. Defaults to `https://api.afinsp.com`.
    pub server_url: Option<String>,

    /// Timeout for HTTP requests.
    ///
    /// The timeout is applied from when the request starts connecting until
    /// the response body has finished. If set to `None`, requests never time
    /// out.
    ///
    /// Default: 15 seconds.
    pub timeout: Option<Duration>,

    /// Number of retries.
    ///
    /// The number of times the client will retry if a server-side error
    /// or timeout is received.
    ///
    /// Default: 2
    pub num_retries: Option<u32>,

    /// Retry Schedule.
    ///
    /// List of delays to wait before each retry attempt.
    /// Takes precedence over `num_retries`.
    pub retry_schedule: Option<Vec<Duration>>,
}

impl Default for AfinspOptions {
    fn default() -> Self {
        Self {
            debug: false,
            server_url: None,
            timeout: Some(Duration::from_secs(15)),
            num_retries: None,
            retry_schedule: None,
        }
    }
}

/// Afinsp API client.
///
/// This is the main entry point for interacting with the Afinsp API.
///
/// # Example
///
/// ```no_run
/// use afinsp_rs::api::{Afinsp, AfinspOptions};
///
/// # async fn example() -> Result<(), afinsp_rs::error::Error> {
/// let client = Afinsp::new("your-api-key".to_string(), None);
///
/// // Access API resources
/// let customers = client.customers().list_customers(None).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Afinsp {
    pub(super) cfg: Arc<Configuration>,
}

impl Afinsp {
    /// Create a new Afinsp client with the given API key.
    ///
    /// # Arguments
    ///
    /// * `token` - Your Afinsp API key
    /// * `options` - Optional configuration options
    pub fn new(token: String, options: Option<AfinspOptions>) -> Self {
        let options = options.unwrap_or_default();

        let cfg = Arc::new(Configuration {
            user_agent: Some(format!("afinsp-rust/{CRATE_VERSION}")),
            client: HyperClient::builder(TokioExecutor::new()).build(crate::make_connector()),
            timeout: options.timeout,
            base_path: options
                .server_url
                .unwrap_or_else(|| "https://api.afinsp.com".to_string()),
            bearer_access_token: Some(token),
            num_retries: options.num_retries.unwrap_or(2),
            retry_schedule: options.retry_schedule,
        });

        Self { cfg }
    }

    /// Creates a new `Afinsp` API client with a different token,
    /// re-using all of the settings and the Hyper client from
    /// an existing `Afinsp` instance.
    ///
    /// This can be used to change the token without incurring
    /// the cost of TLS initialization.
    pub fn with_token(&self, token: String) -> Self {
        let cfg = Arc::new(Configuration {
            base_path: self.cfg.base_path.clone(),
            user_agent: self.cfg.user_agent.clone(),
            bearer_access_token: Some(token),
            client: self.cfg.client.clone(),
            timeout: self.cfg.timeout,
            num_retries: self.cfg.num_retries,
            retry_schedule: self.cfg.retry_schedule.clone(),
        });

        Self { cfg }
    }

    /// Access the add ons API.
    pub fn add_ons(&self) -> super::AddOns<'_> {
        super::AddOns::new(&self.cfg)
    }

    /// Access the batch jobs API.
    pub fn batch_jobs(&self) -> super::BatchJobs<'_> {
        super::BatchJobs::new(&self.cfg)
    }

    /// Access the checkout sessions API.
    pub fn checkout_sessions(&self) -> super::CheckoutSessions<'_> {
        super::CheckoutSessions::new(&self.cfg)
    }

    /// Access the connect API.
    pub fn connect(&self) -> super::Connect<'_> {
        super::Connect::new(&self.cfg)
    }

    /// Access the coupons API.
    pub fn coupons(&self) -> super::Coupons<'_> {
        super::Coupons::new(&self.cfg)
    }

    /// Access the credit notes API.
    pub fn credit_notes(&self) -> super::CreditNotes<'_> {
        super::CreditNotes::new(&self.cfg)
    }

    /// Access the custom properties API.
    pub fn custom_properties(&self) -> super::CustomProperties<'_> {
        super::CustomProperties::new(&self.cfg)
    }

    /// Access the customers API.
    pub fn customers(&self) -> super::Customers<'_> {
        super::Customers::new(&self.cfg)
    }

    /// Access the events API.
    pub fn events(&self) -> super::Events<'_> {
        super::Events::new(&self.cfg)
    }

    /// Access the features API.
    pub fn features(&self) -> super::Features<'_> {
        super::Features::new(&self.cfg)
    }

    /// Access the invoices API.
    pub fn invoices(&self) -> super::Invoices<'_> {
        super::Invoices::new(&self.cfg)
    }

    /// Access the metrics API.
    pub fn metrics(&self) -> super::Metrics<'_> {
        super::Metrics::new(&self.cfg)
    }

    /// Access the o auth API.
    pub fn o_auth(&self) -> super::OAuth<'_> {
        super::OAuth::new(&self.cfg)
    }

    /// Access the o auth apps API.
    pub fn o_auth_apps(&self) -> super::OAuthApps<'_> {
        super::OAuthApps::new(&self.cfg)
    }

    /// Access the plans API.
    pub fn plans(&self) -> super::Plans<'_> {
        super::Plans::new(&self.cfg)
    }

    /// Access the product families API.
    pub fn product_families(&self) -> super::ProductFamilies<'_> {
        super::ProductFamilies::new(&self.cfg)
    }

    /// Access the products API.
    pub fn products(&self) -> super::Products<'_> {
        super::Products::new(&self.cfg)
    }

    /// Access the subscriptions API.
    pub fn subscriptions(&self) -> super::Subscriptions<'_> {
        super::Subscriptions::new(&self.cfg)
    }

    /// Access the usage API.
    pub fn usage(&self) -> super::Usage<'_> {
        super::Usage::new(&self.cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::Afinsp;

    #[test]
    fn test_future_send_sync() {
        fn require_send_sync<T: Send + Sync>(_: T) {}

        let afinsp = Afinsp::new(String::new(), None);
        let customer_api = afinsp.customers();
        let fut = customer_api.list_customers(None);
        require_send_sync(fut);
    }
}
