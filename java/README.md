# Afinsp Java SDK

Official Java SDK for the [Afinsp](https://afinsp.com) billing platform.

## Requirements

- Java 11 or later

## Installation

### Gradle

```groovy
dependencies {
    implementation 'com.afinsp:afinsp-java:0.1.0'
}
```

### Maven

```xml
<dependency>
    <groupId>com.afinsp</groupId>
    <artifactId>afinsp-java</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Quick Start

```java
import com.afinsp.Afinsp;
import com.afinsp.AfinspOptions;
import com.afinsp.models.*;

// Initialize the client
Afinsp afinsp = new Afinsp("your-api-key");

// Or with custom options
AfinspOptions options = new AfinspOptions();
options.setServerUrl("https://api.afinsp.com");
Afinsp afinsp = new Afinsp("your-api-key", options);
```

## Usage

### Customers

```java
// List customers
CustomerListResponse customers = afinsp.getCustomers().listCustomers();

// List with options
CustomersListCustomersOptions options = new CustomersListCustomersOptions();
options.page = 1;
options.perPage = 20;
options.search = "acme";
CustomerListResponse customers = afinsp.getCustomers().listCustomers(options);

// Create a customer
CustomerCreateRequest request = new CustomerCreateRequest()
    .name("Acme Corp")
    .alias("acme-corp")
    .billingEmail("billing@acme.com");
Customer customer = afinsp.getCustomers().createCustomer(request);

// Get a customer by ID or alias
Customer customer = afinsp.getCustomers().getCustomer("acme-corp");

// Update a customer
CustomerUpdateRequest updateRequest = new CustomerUpdateRequest()
    .name("Acme Corporation");
Customer updated = afinsp.getCustomers().updateCustomer("acme-corp", updateRequest);

// Archive a customer (all subscriptions must be terminated first)
afinsp.getCustomers().archiveCustomer("acme-corp");

// Generate a customer portal token
CustomerPortalTokenResponse token = afinsp.getCustomers().createPortalToken("acme-corp");
```

### Subscriptions

```java
// List subscriptions
SubscriptionListResponse subscriptions = afinsp.getSubscriptions().listSubscriptions();

// Create a subscription
SubscriptionCreateRequest request = new SubscriptionCreateRequest()
    .customerIdOrAlias("cust_123")
    .planId("plan_456")
    .billingDayAnchor(1);
SubscriptionDetails subscription = afinsp.getSubscriptions().createSubscription(request);

// Get subscription details
SubscriptionDetails details = afinsp.getSubscriptions().subscriptionDetails("sub_789");

// Cancel a subscription
CancelSubscriptionRequest cancelRequest = new CancelSubscriptionRequest()
    .reason("Customer requested");
afinsp.getSubscriptions().cancelSubscription("sub_789", cancelRequest);
```

### Plans

```java
// List plans
PlanListResponse plans = afinsp.getPlans().listPlans();

// Get a plan by ID or alias
Plan plan = afinsp.getPlans().getPlanDetails("plan_123");
```

### Invoices

```java
// List invoices
InvoiceListResponse invoices = afinsp.getInvoices().listInvoices();

// Get an invoice
Invoice invoice = afinsp.getInvoices().getInvoiceById("inv_123");

// Download invoice as PDF
byte[] pdfBytes = afinsp.getInvoices().downloadInvoicePdf("inv_123");
```

### Product Families

```java
// List product families
ProductFamilyListResponse families = afinsp.getProductFamilies().listProductFamilies();

// Create a product family
ProductFamilyCreateRequest request = new ProductFamilyCreateRequest()
    .name("SaaS Products");
ProductFamily family = afinsp.getProductFamilies().createProductFamily(request);
```

### Events (Usage Tracking)

```java
// Ingest usage events
IngestEventsRequest request = new IngestEventsRequest()
    .events(Arrays.asList(
        new Event()
            .code("api_call")
            .customerId("cust_123")
            .timestamp(Instant.now().toString())
            .properties(Map.of("endpoint", "/users", "method", "GET"))
    ));
IngestEventsResponse response = afinsp.getEvents().ingestEvents(request);
```

### Checkout Sessions

```java
// Create a checkout session
CreateCheckoutSessionRequest request = new CreateCheckoutSessionRequest()
    .customerId("cust_123")
    .planVersionId("pv_456");
CreateCheckoutSessionResponse session = afinsp.getCheckoutSessions().createCheckoutSession(request);

// Get a checkout session
GetCheckoutSessionResponse session = afinsp.getCheckoutSessions().getCheckoutSession("cs_789");

// Cancel a checkout session
afinsp.getCheckoutSessions().cancelCheckoutSession("cs_789");
```

## Webhook Verification

The SDK provides utilities for verifying webhook signatures using the [Standard Webhooks](https://www.standardwebhooks.com/) specification. Afinsp uses Svix for webhook delivery, so the SDK supports both `svix-*` headers and `webhook-*` headers.

**Supported headers:**

- `svix-id`, `svix-timestamp`, `svix-signature` (Svix-branded)
- `webhook-id`, `webhook-timestamp`, `webhook-signature` (Standard Webhooks)

```java
import com.afinsp.Webhook;
import com.standardwebhooks.exceptions.WebhookVerificationException;

// Initialize with your webhook signing secret
Webhook webhook = new Webhook("whsec_your_secret_here");

// In your webhook endpoint handler:
try {
    // payload is the raw request body as a string
    // headers are the HTTP headers from the request
    webhook.verify(payload, headers);

    // Signature is valid, process the webhook
    // ...
} catch (WebhookVerificationException e) {
    // Invalid signature - reject the request
    return ResponseEntity.status(401).body("Invalid signature");
}
```

### Using with Spring Boot

```java
@RestController
public class WebhookController {
    private final Webhook webhook = new Webhook("whsec_your_secret");

    @PostMapping("/webhooks/afinsp")
    public ResponseEntity<String> handleWebhook(
            @RequestBody String payload,
            @RequestHeader Map<String, String> headerMap) {

        try {
            // Convert headers to the expected format
            Map<String, List<String>> headers = new HashMap<>();
            headerMap.forEach((k, v) -> headers.put(k.toLowerCase(), List.of(v)));

            webhook.verify(payload, headers);

            // Process the webhook event
            JsonNode event = objectMapper.readTree(payload);
            String eventType = event.get("type").asText();

            switch (eventType) {
                case "customer.created":
                    handleCustomerCreated(event);
                    break;
                case "invoice.finalized":
                    handleInvoiceFinalized(event);
                    break;
                // ... handle other event types
            }

            return ResponseEntity.ok("OK");
        } catch (WebhookVerificationException e) {
            return ResponseEntity.status(401).body("Invalid signature");
        }
    }
}
```

## Error Handling

The SDK throws `ApiException` for API errors:

```java
import com.afinsp.exceptions.ApiException;

try {
    Customer customer = afinsp.getCustomers().getCustomer("nonexistent");
} catch (ApiException e) {
    System.out.println("Status code: " + e.getCode());
    System.out.println("Error message: " + e.getMessage());
    System.out.println("Response body: " + e.getResponseBody());
} catch (IOException e) {
    System.out.println("Network error: " + e.getMessage());
}
```

## Configuration

### Custom Server URL

```java
AfinspOptions options = new AfinspOptions();
options.setServerUrl("https://afinsp-api.example.com");
Afinsp afinsp = new Afinsp("your-api-key", options);
```

### Retry Configuration

The SDK automatically retries requests on 5xx server errors with delays of 50ms, 100ms, and 200ms.

## Building from Source

```bash
# Build the project
./gradlew build

# Run tests
./gradlew test
```

## Code Generation

The API client code is generated from the OpenAPI specification. To regenerate:

```bash
# Build the codegen Docker image (from repo root)
cd codegen && docker build -t afinsp-codegen:latest .

# Run code generation (from repo root)
./regen_openapi.py
```

## License

Apache License 2.0
