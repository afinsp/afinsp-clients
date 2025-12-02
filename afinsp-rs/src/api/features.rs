// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct FeaturesListFeaturesOptions {
    /// Filter by feature status. Repeat the param to select multiple, omit to return all.
    pub statuses: Option<Vec<FeatureStatus>>,

    /// Filter by product. Omit to return features across all products.
    pub product_id: Option<ProductId>,

    /// Search by feature name.
    pub search: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct Features<'a> {
    cfg: &'a Configuration,
}

impl<'a> Features<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_features(
        &self,
        options: Option<FeaturesListFeaturesOptions>,
    ) -> Result<crate::models::FeatureListResponse> {
        let FeaturesListFeaturesOptions {
            statuses,
            product_id,
            search,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/features")
            .with_optional_exploded_query_param("statuses", statuses)
            .with_optional_query_param("product_id", product_id)
            .with_optional_query_param("search", search)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    pub async fn get_feature(&self, id_or_code: String) -> Result<crate::models::Feature> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/features/{id_or_code}")
            .with_path_param("id_or_code", id_or_code)
            .execute(self.cfg)
            .await
    }
}
