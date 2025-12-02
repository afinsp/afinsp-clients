// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ProductFamiliesListProductFamiliesOptions {
    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,

    pub search: Option<String>,
}

pub struct ProductFamilies<'a> {
    cfg: &'a Configuration,
}

impl<'a> ProductFamilies<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_product_families(
        &self,
        options: Option<ProductFamiliesListProductFamiliesOptions>,
    ) -> Result<crate::models::ProductFamilyListResponse> {
        let ProductFamiliesListProductFamiliesOptions {
            order_by,
            page,
            per_page,
            search,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/product_families")
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_optional_query_param("search", search)
            .execute(self.cfg)
            .await
    }

    pub async fn create_product_family(
        &self,
        product_family_create_request: crate::models::ProductFamilyCreateRequest,
    ) -> Result<crate::models::ProductFamily> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/product_families")
            .with_body_param(product_family_create_request)
            .execute(self.cfg)
            .await
    }

    /// Retrieve a single product family by ID or alias.
    pub async fn get_product_family_by_id_or_alias(
        &self,
        id_or_alias: String,
    ) -> Result<crate::models::ProductFamily> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/product_families/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .execute(self.cfg)
            .await
    }
}
