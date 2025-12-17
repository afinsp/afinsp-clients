use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde_json::json;

use afinsp_rs::models::{
    Currency, Customer, CustomerCreateRequest, CustomerListResponse, Fee, InvoiceStatus,
    PaginationResponse, RatePlanFee,
};

/// Helper to test JSON deserialization
fn assert_deserializes_to<T: Debug + PartialEq + DeserializeOwned>(
    value: serde_json::Value,
    expected: T,
) {
    let actual: T = serde_json::from_value(value).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_currency_serialization() {
    // Test serialization
    assert_eq!(json!(Currency::Usd), json!("USD"));
    assert_eq!(json!(Currency::Eur), json!("EUR"));
    assert_eq!(json!(Currency::Gbp), json!("GBP"));

    // Test deserialization
    let usd: Currency = serde_json::from_str(r#""USD""#).unwrap();
    assert_eq!(usd, Currency::Usd);

    let eur: Currency = serde_json::from_str(r#""EUR""#).unwrap();
    assert_eq!(eur, Currency::Eur);
}

#[test]
fn test_currency_display() {
    assert_eq!(Currency::Usd.to_string(), "USD");
    assert_eq!(Currency::Eur.to_string(), "EUR");
    assert_eq!(Currency::Jpy.to_string(), "JPY");
}

#[test]
fn test_invoice_status_serialization() {
    assert_eq!(json!(InvoiceStatus::Draft), json!("DRAFT"));
    assert_eq!(json!(InvoiceStatus::Finalized), json!("FINALIZED"));
    assert_eq!(json!(InvoiceStatus::Void), json!("VOID"));

    let draft: InvoiceStatus = serde_json::from_str(r#""DRAFT""#).unwrap();
    assert_eq!(draft, InvoiceStatus::Draft);
}

#[test]
fn test_customer_list_response() {
    let json_str = r#"{
        "data": [],
        "pagination_meta": {
            "page": 0,
            "per_page": 10,
            "total_items": 0,
            "total_pages": 0
        }
    }"#;

    let response: CustomerListResponse = serde_json::from_str(json_str).unwrap();
    assert_eq!(response.data.len(), 0);
    assert_eq!(response.pagination_meta.total_items, 0);
}

#[test]
fn test_customer_list_response_with_data() {
    assert_deserializes_to(
        json!({
            "data": [{
                "id": "cust_123",
                "name": "Test Customer",
                "currency": "USD",
                "custom_properties": {},
                "custom_taxes": [],
                "invoicing_emails": [],
                "invoicing_entity_id": "inv_entity_1"
            }],
            "pagination_meta": {
                "page": 0,
                "per_page": 10,
                "total_items": 1,
                "total_pages": 1
            }
        }),
        CustomerListResponse {
            data: vec![Customer {
                id: "cust_123".to_string(),
                name: "Test Customer".to_string(),
                currency: Currency::Usd,
                custom_properties: json!({}),
                custom_taxes: vec![],
                invoicing_emails: vec![],
                invoicing_entity_id: "inv_entity_1".to_string(),
                ..Default::default()
            }],
            pagination_meta: PaginationResponse {
                page: 0,
                per_page: 10,
                total_items: 1,
                total_pages: 1,
            },
        },
    );
}

#[test]
fn test_customer_create_request_serialization() {
    let request = CustomerCreateRequest::new(
        Currency::Usd,
        vec![],
        vec!["billing@example.com".to_string()],
        "Acme Corp".to_string(),
    );

    let json_value = serde_json::to_value(&request).unwrap();

    assert_eq!(json_value["name"], "Acme Corp");
    assert_eq!(json_value["currency"], "USD");
    assert_eq!(
        json_value["invoicing_emails"],
        json!(["billing@example.com"])
    );

    // Optional fields should not be present when None
    assert!(json_value.get("alias").is_none());
    assert!(json_value.get("billing_email").is_none());
}

#[test]
fn test_customer_create_request_with_optional_fields() {
    let mut request =
        CustomerCreateRequest::new(Currency::Eur, vec![], vec![], "Test Company".to_string());
    request.alias = Some("test-alias".to_string());
    request.billing_email = Some("billing@test.com".to_string());

    let json_value = serde_json::to_value(&request).unwrap();

    assert_eq!(json_value["alias"], "test-alias");
    assert_eq!(json_value["billing_email"], "billing@test.com");
}

#[test]
fn test_fee_tagged_union_serialization() {
    // Test the internally tagged union (discriminator: type)
    let rate_fee = Fee::Rate(RatePlanFee { rates: vec![] });

    let json_value = serde_json::to_value(&rate_fee).unwrap();
    assert_eq!(json_value["type"], "RATE");
}

#[test]
fn test_fee_tagged_union_deserialization() {
    let json_str = r#"{
        "type": "RATE",
        "rates": []
    }"#;

    let fee: Fee = serde_json::from_str(json_str).unwrap();
    match fee {
        Fee::Rate(rate) => {
            assert!(rate.rates.is_empty());
        }
        _ => panic!("Expected Rate variant"),
    }
}

#[test]
fn test_unknown_fields_are_ignored() {
    // API responses may include fields we don't know about yet
    let json_str = r#"{
        "data": [],
        "pagination_meta": {
            "page": 0,
            "per_page": 10,
            "total_items": 0,
            "total_pages": 0
        },
        "unknown_field": "should be ignored"
    }"#;

    let response: CustomerListResponse = serde_json::from_str(json_str).unwrap();
    assert_eq!(response.data.len(), 0);
}

#[test]
fn test_optional_fields_deserialize_as_none() {
    let json_str = r#"{
        "id": "cust_123",
        "name": "Test",
        "currency": "USD",
        "custom_properties": {},
        "custom_taxes": [],
        "invoicing_emails": [],
        "invoicing_entity_id": "inv_1"
    }"#;

    let customer: Customer = serde_json::from_str(json_str).unwrap();
    assert!(customer.alias.is_none());
    assert!(customer.billing_email.is_none());
    assert!(customer.phone.is_none());
}

#[test]
fn test_optional_fields_deserialize_as_some() {
    let json_str = r#"{
        "id": "cust_123",
        "name": "Test",
        "currency": "USD",
        "custom_properties": {},
        "custom_taxes": [],
        "invoicing_emails": [],
        "invoicing_entity_id": "inv_1",
        "alias": "my-alias",
        "billing_email": "billing@example.com"
    }"#;

    let customer: Customer = serde_json::from_str(json_str).unwrap();
    assert_eq!(customer.alias, Some("my-alias".to_string()));
    assert_eq!(
        customer.billing_email,
        Some("billing@example.com".to_string())
    );
}
