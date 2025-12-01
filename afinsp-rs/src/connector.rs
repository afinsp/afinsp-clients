//! HTTP connector with TLS support.

use std::{future::Future, pin::Pin};

use http1::Uri;
use hyper_util::client::legacy::connect::HttpConnector;
use tokio::net::TcpStream;
use tower_service::Service;

#[cfg(feature = "rustls-tls")]
use hyper_util::rt::TokioIo;

// If no TLS backend is enabled, use plain http connector.
#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
type HttpsIfAvailable<T> = T;
#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
type MaybeHttpsStream<T> = T;

// If only native TLS is enabled, use that.
#[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
use hyper_tls::{HttpsConnector as HttpsIfAvailable, MaybeHttpsStream};

// If rustls is enabled, use that.
#[cfg(feature = "rustls-tls")]
use hyper_rustls::{HttpsConnector as HttpsIfAvailable, MaybeHttpsStream};

fn wrap_connector<H>(conn: H) -> HttpsIfAvailable<H> {
    #[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
    return conn;

    #[cfg(feature = "rustls-tls")]
    {
        let crypto_provider = rustls::crypto::CryptoProvider::get_default()
            .map(std::sync::Arc::clone)
            .unwrap_or_else(|| std::sync::Arc::new(rustls::crypto::ring::default_provider()));

        let builder = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_native_roots(crypto_provider)
            .unwrap()
            .https_or_http();

        #[cfg(feature = "http1")]
        let builder = builder.enable_http1();

        #[cfg(feature = "http2")]
        let builder = builder.enable_http2();

        builder.wrap_connector(conn)
    }

    #[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
    return hyper_tls::HttpsConnector::new_with_connector(conn);
}

#[derive(Clone, Debug)]
pub(crate) struct Connector(HttpsIfAvailable<HttpConnector>);

pub(crate) fn make_connector() -> Connector {
    let mut http = hyper_util::client::legacy::connect::HttpConnector::new();
    if cfg!(any(feature = "native-tls", feature = "rustls-tls")) {
        http.enforce_http(false);
    }

    Connector(wrap_connector(http))
}

#[cfg(feature = "rustls-tls")]
impl Service<Uri> for Connector {
    type Response = MaybeHttpsStream<TokioIo<TcpStream>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        Box::pin(self.0.call(req))
    }
}

#[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
impl Service<Uri> for Connector {
    type Response = MaybeHttpsStream<TcpStream>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        Box::pin(self.0.call(req))
    }
}

#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
impl Service<Uri> for Connector {
    type Response = TcpStream;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        Box::pin(self.0.call(req))
    }
}
