// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MetricsListMetricsOptions {
    pub product_family_id: Option<ProductFamilyId>,

    /// Search by metric name or code
    pub search: Option<String>,

    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `code`, `created_at`. Direction: `asc` or `desc`. Default: `name.asc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct Metrics<'a> {
    cfg: &'a Configuration,
}

impl<'a> Metrics<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_metrics(
        &self,
        options: Option<MetricsListMetricsOptions>,
    ) -> Result<crate::models::MetricListResponse> {
        let MetricsListMetricsOptions {
            product_family_id,
            search,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/metrics")
            .with_optional_query_param("product_family_id", product_family_id)
            .with_optional_query_param("search", search)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    pub async fn create_metric(
        &self,
        create_metric_request: crate::models::CreateMetricRequest,
    ) -> Result<crate::models::Metric> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/metrics")
            .with_body_param(create_metric_request)
            .execute(self.cfg)
            .await
    }

    pub async fn get_metric(&self, metric_id: String) -> Result<crate::models::Metric> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/metrics/{metric_id}")
            .with_path_param("metric_id", metric_id)
            .execute(self.cfg)
            .await
    }

    /// Partially update metric fields. Code and aggregation_type are immutable.
    pub async fn update_metric(
        &self,
        metric_id: String,
        update_metric_request: crate::models::UpdateMetricRequest,
    ) -> Result<crate::models::Metric> {
        crate::request::Request::new(http1::Method::PATCH, "/api/v1/metrics/{metric_id}")
            .with_path_param("metric_id", metric_id)
            .with_body_param(update_metric_request)
            .execute(self.cfg)
            .await
    }

    pub async fn archive_metric(&self, metric_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/metrics/{metric_id}/archive")
            .with_path_param("metric_id", metric_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    pub async fn unarchive_metric(&self, metric_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/metrics/{metric_id}/unarchive")
            .with_path_param("metric_id", metric_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }
}
