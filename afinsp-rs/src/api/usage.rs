// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct UsageGetCustomerUsageOptions {
    pub start_date: String,

    pub end_date: String,

    pub metric_id: Option<BillableMetricId>,
}

#[derive(Default)]
pub struct UsageGetSubscriptionUsageOptions {
    pub start_date: Option<String>,

    pub end_date: Option<String>,

    pub metric_id: Option<BillableMetricId>,
}

#[derive(Default)]
pub struct UsageGetUsageSummaryOptions {
    pub start_date: String,

    pub end_date: String,

    pub metric_id: Option<BillableMetricId>,
}

pub struct Usage<'a> {
    cfg: &'a Configuration,
}

impl<'a> Usage<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Retrieve aggregated usage data for a customer over a specified period.
    pub async fn get_customer_usage(
        &self,
        customer_id: String,
        options: UsageGetCustomerUsageOptions,
    ) -> Result<crate::models::UsageResponse> {
        let UsageGetCustomerUsageOptions {
            start_date,
            end_date,
            metric_id,
        } = options;

        crate::request::Request::new(http1::Method::GET, "/api/v1/usage/customer/{customer_id}")
            .with_path_param("customer_id", customer_id)
            .with_query_param("start_date", start_date)
            .with_query_param("end_date", end_date)
            .with_optional_query_param("metric_id", metric_id)
            .execute(self.cfg)
            .await
    }

    /// Retrieve aggregated usage data for a subscription's usage-based components.
    /// If start_date/end_date are omitted, defaults to the current billing period.
    pub async fn get_subscription_usage(
        &self,
        subscription_id: String,
        options: Option<UsageGetSubscriptionUsageOptions>,
    ) -> Result<crate::models::UsageResponse> {
        let UsageGetSubscriptionUsageOptions {
            start_date,
            end_date,
            metric_id,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/usage/subscription/{subscription_id}",
        )
        .with_path_param("subscription_id", subscription_id)
        .with_optional_query_param("start_date", start_date)
        .with_optional_query_param("end_date", end_date)
        .with_optional_query_param("metric_id", metric_id)
        .execute(self.cfg)
        .await
    }

    /// Retrieve aggregated usage data across all customers for the tenant.
    pub async fn get_usage_summary(
        &self,
        options: UsageGetUsageSummaryOptions,
    ) -> Result<crate::models::UsageResponse> {
        let UsageGetUsageSummaryOptions {
            start_date,
            end_date,
            metric_id,
        } = options;

        crate::request::Request::new(http1::Method::GET, "/api/v1/usage/summary")
            .with_query_param("start_date", start_date)
            .with_query_param("end_date", end_date)
            .with_optional_query_param("metric_id", metric_id)
            .execute(self.cfg)
            .await
    }
}
