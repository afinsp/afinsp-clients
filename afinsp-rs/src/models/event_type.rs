// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum EventType {
    #[default]
    #[serde(rename = "metric.created")]
    MetricCreated,

    #[serde(rename = "customer.created")]
    CustomerCreated,

    #[serde(rename = "subscription.created")]
    SubscriptionCreated,

    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,

    #[serde(rename = "subscription.cancelled")]
    SubscriptionCancelled,

    #[serde(rename = "subscription.ended")]
    SubscriptionEnded,

    #[serde(rename = "invoice.created")]
    InvoiceCreated,

    #[serde(rename = "invoice.finalized")]
    InvoiceFinalized,

    #[serde(rename = "invoice.paid")]
    InvoicePaid,

    #[serde(rename = "invoice.voided")]
    InvoiceVoided,

    #[serde(rename = "invoice.consolidated")]
    InvoiceConsolidated,

    #[serde(rename = "quote.accepted")]
    QuoteAccepted,

    #[serde(rename = "quote.converted")]
    QuoteConverted,

    #[serde(rename = "credit_note.created")]
    CreditNoteCreated,

    #[serde(rename = "credit_note.finalized")]
    CreditNoteFinalized,

    #[serde(rename = "credit_note.voided")]
    CreditNoteVoided,

    #[serde(rename = "plan.created")]
    PlanCreated,

    #[serde(rename = "plan.published")]
    PlanPublished,

    #[serde(rename = "plan.archived")]
    PlanArchived,

    #[serde(rename = "product.created")]
    ProductCreated,

    #[serde(rename = "product.updated")]
    ProductUpdated,

    #[serde(rename = "product.archived")]
    ProductArchived,

    #[serde(rename = "metric.updated")]
    MetricUpdated,

    #[serde(rename = "metric.archived")]
    MetricArchived,

    #[serde(rename = "coupon.created")]
    CouponCreated,

    #[serde(rename = "coupon.updated")]
    CouponUpdated,

    #[serde(rename = "coupon.archived")]
    CouponArchived,

    #[serde(rename = "addon.created")]
    AddonCreated,

    #[serde(rename = "addon.updated")]
    AddonUpdated,

    #[serde(rename = "addon.archived")]
    AddonArchived,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::MetricCreated => "metric.created",
            Self::CustomerCreated => "customer.created",
            Self::SubscriptionCreated => "subscription.created",
            Self::SubscriptionUpdated => "subscription.updated",
            Self::SubscriptionCancelled => "subscription.cancelled",
            Self::SubscriptionEnded => "subscription.ended",
            Self::InvoiceCreated => "invoice.created",
            Self::InvoiceFinalized => "invoice.finalized",
            Self::InvoicePaid => "invoice.paid",
            Self::InvoiceVoided => "invoice.voided",
            Self::InvoiceConsolidated => "invoice.consolidated",
            Self::QuoteAccepted => "quote.accepted",
            Self::QuoteConverted => "quote.converted",
            Self::CreditNoteCreated => "credit_note.created",
            Self::CreditNoteFinalized => "credit_note.finalized",
            Self::CreditNoteVoided => "credit_note.voided",
            Self::PlanCreated => "plan.created",
            Self::PlanPublished => "plan.published",
            Self::PlanArchived => "plan.archived",
            Self::ProductCreated => "product.created",
            Self::ProductUpdated => "product.updated",
            Self::ProductArchived => "product.archived",
            Self::MetricUpdated => "metric.updated",
            Self::MetricArchived => "metric.archived",
            Self::CouponCreated => "coupon.created",
            Self::CouponUpdated => "coupon.updated",
            Self::CouponArchived => "coupon.archived",
            Self::AddonCreated => "addon.created",
            Self::AddonUpdated => "addon.updated",
            Self::AddonArchived => "addon.archived",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for EventType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
