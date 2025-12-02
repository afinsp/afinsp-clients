// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ProductsListProductsOptions {
    pub product_family_id: Option<ProductFamilyId>,

    pub search: Option<String>,

    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `created_at`. Direction: `asc` or `desc`. Default: `name.asc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct Products<'a> {
    cfg: &'a Configuration,
}

impl<'a> Products<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_products(
        &self,
        options: Option<ProductsListProductsOptions>,
    ) -> Result<crate::models::ProductListResponse> {
        let ProductsListProductsOptions {
            product_family_id,
            search,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/products")
            .with_optional_query_param("product_family_id", product_family_id)
            .with_optional_query_param("search", search)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    pub async fn create_product(
        &self,
        create_product_request: crate::models::CreateProductRequest,
    ) -> Result<crate::models::Product> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/products")
            .with_body_param(create_product_request)
            .execute(self.cfg)
            .await
    }

    pub async fn get_product(&self, product_id: String) -> Result<crate::models::Product> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/products/{product_id}")
            .with_path_param("product_id", product_id)
            .execute(self.cfg)
            .await
    }

    /// Partially update product fields. The fee_type is immutable and cannot be changed.
    pub async fn update_product(
        &self,
        product_id: String,
        update_product_request: crate::models::UpdateProductRequest,
    ) -> Result<crate::models::Product> {
        crate::request::Request::new(http1::Method::PATCH, "/api/v1/products/{product_id}")
            .with_path_param("product_id", product_id)
            .with_body_param(update_product_request)
            .execute(self.cfg)
            .await
    }

    pub async fn archive_product(&self, product_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/products/{product_id}/archive")
            .with_path_param("product_id", product_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    pub async fn list_product_entitlements(
        &self,
        product_id: String,
    ) -> Result<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/products/{product_id}/entitlements",
        )
        .with_path_param("product_id", product_id)
        .execute(self.cfg)
        .await
    }

    pub async fn unarchive_product(&self, product_id: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/products/{product_id}/unarchive",
        )
        .with_path_param("product_id", product_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
