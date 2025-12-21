# AFINSP Rust SDK

Official Rust SDK for the [AFINSP](https://afinsp.com) billing API.

## Installation

Add `afinsp-rs` to your `Cargo.toml`:

```toml
[dependencies]
afinsp-rs = "0.1.1"
```

## Quick Start

```rust
use afinsp_rs::api::{Afinsp, AfinspOptions};

#[tokio::main]
async fn main() -> Result<(), afinsp_rs::error::Error> {
    // Create a client with your API key
    let client = Afinsp::new("your-api-key".to_string(), None);

    // List customers
    let customers = client.customers().list_customers(None).await?;
    println!("Found {} customers", customers.data.len());

    Ok(())
}
```

## Available APIs

The SDK provides access to the following AFINSP API resources:

| Resource                     | Description                       |
| ---------------------------- | --------------------------------- |
| `client.customers()`         | Manage customers                  |
| `client.subscriptions()`     | Manage subscriptions              |
| `client.invoices()`          | Access invoices and download PDFs |
| `client.plans()`             | List available plans              |
| `client.product_families()`  | Manage product families           |
| `client.events()`            | Send usage events                 |
| `client.checkout_sessions()` | Create checkout sessions          |

## Examples

### Creating a Customer

```rust
use afinsp_rs::api::Afinsp;
use afinsp_rs::models::CustomerCreateRequest;

let client = Afinsp::new("your-api-key".to_string(), None);

let customer = client.customers().create_customer(CustomerCreateRequest {
    name: "Acme Corp".to_string(),
    alias: Some("acme".to_string()),
    email: Some("billing@acme.com".to_string()),
    ..Default::default()
}).await?;

println!("Created customer: {}", customer.id);
```

### Listing Customers with Pagination

```rust
use afinsp_rs::api::{Afinsp, CustomersListCustomersOptions};

let client = Afinsp::new("your-api-key".to_string(), None);

let customers = client.customers().list_customers(Some(CustomersListCustomersOptions {
    page: Some(0),
    per_page: Some(10),
    search: Some("acme".to_string()),
    ..Default::default()
})).await?;

for customer in customers.data {
    println!("Customer: {} ({})", customer.name, customer.id);
}
```

### Downloading an Invoice PDF

```rust
use afinsp_rs::api::Afinsp;

let client = Afinsp::new("your-api-key".to_string(), None);

let pdf_bytes = client.invoices()
    .download_invoice_pdf("invoice_id".to_string())
    .await?;

// Save to file
std::fs::write("invoice.pdf", &pdf_bytes)?;
```

### Sending Usage Events

```rust
use afinsp_rs::api::Afinsp;
use afinsp_rs::models::{IngestEventsRequest, Event};
use std::collections::HashMap;

let client = Afinsp::new("your-api-key".to_string(), None);

let mut properties = HashMap::new();
properties.insert("endpoint".to_string(), "/api/v1/users".to_string());
properties.insert("method".to_string(), "GET".to_string());

client.events().ingest_events(IngestEventsRequest {
    events: vec![Event {
        code: "api_call".to_string(),
        customer_id: "customer_id".to_string(),
        event_id: "unique_event_id".to_string(),
        timestamp: "2024-01-15T10:30:00Z".to_string(),
        properties: Some(properties),
    }],
    allow_backfilling: None,
}).await?;
```

## Configuration

### Custom Server URL

For self-hosted AFINSP instances:

```rust
use afinsp_rs::api::{Afinsp, AfinspOptions};

let options = AfinspOptions {
    server_url: Some("https://your-afinsp-instance.com".to_string()),
    ..Default::default()
};

let client = Afinsp::new("your-api-key".to_string(), Some(options));
```

### Timeout and Retries

```rust
use afinsp_rs::api::{Afinsp, AfinspOptions};
use std::time::Duration;

let options = AfinspOptions {
    timeout: Some(Duration::from_secs(30)),
    num_retries: Some(3),
    ..Default::default()
};

let client = Afinsp::new("your-api-key".to_string(), Some(options));
```

### Custom Retry Schedule

```rust
use afinsp_rs::api::{Afinsp, AfinspOptions};
use std::time::Duration;

let options = AfinspOptions {
    retry_schedule: Some(vec![
        Duration::from_millis(100),
        Duration::from_millis(500),
        Duration::from_secs(1),
    ]),
    ..Default::default()
};

let client = Afinsp::new("your-api-key".to_string(), Some(options));
```

## Cargo Features

| Feature      | Description                          | Default |
| ------------ | ------------------------------------ | ------- |
| `rustls-tls` | Use rustls for TLS                   | Yes     |
| `native-tls` | Use native OS TLS (OpenSSL on Linux) | No      |
| `http1`      | HTTP/1.1 support                     | Yes     |
| `http2`      | HTTP/2 support                       | No      |

### Using Native TLS

```toml
[dependencies]
afinsp-rs = { version = "0.1.1", default-features = false, features = ["native-tls", "http1"] }
```

## Error Handling

All API methods return `Result<T, afinsp_rs::error::Error>`. The `Error` type provides detailed information about what went wrong:

```rust
use afinsp_rs::api::Afinsp;
use afinsp_rs::error::Error;

let client = Afinsp::new("your-api-key".to_string(), None);

match client.customers().get_customer("invalid-id".to_string()).await {
    Ok(customer) => println!("Found: {}", customer.name),
    Err(Error::Http(e)) => {
        println!("HTTP error {}: {}", e.status, e.body.message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Thread Safety

The `Afinsp` client is both `Send` and `Sync`, making it safe to share across threads:

```rust
use afinsp_rs::api::Afinsp;
use std::sync::Arc;

let client = Arc::new(Afinsp::new("your-api-key".to_string(), None));

// Clone the Arc for use in different tasks
let client_clone = Arc::clone(&client);
tokio::spawn(async move {
    let customers = client_clone.customers().list_customers(None).await;
});
```

## License

MIT
