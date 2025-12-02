// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct CustomersListCustomersOptions {
    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `email`, `alias`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,

    pub search: Option<String>,

    pub archived: Option<bool>,
}

pub struct Customers<'a> {
    cfg: &'a Configuration,
}

impl<'a> Customers<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List customers with optional pagination and search filtering.
    pub async fn list_customers(
        &self,
        options: Option<CustomersListCustomersOptions>,
    ) -> Result<crate::models::CustomerListResponse> {
        let CustomersListCustomersOptions {
            order_by,
            page,
            per_page,
            search,
            archived,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/customers")
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .with_optional_query_param("search", search)
            .with_optional_query_param("archived", archived)
            .execute(self.cfg)
            .await
    }

    pub async fn create_customer(
        &self,
        customer_create_request: crate::models::CustomerCreateRequest,
    ) -> Result<crate::models::Customer> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/customers")
            .with_body_param(customer_create_request)
            .execute(self.cfg)
            .await
    }

    /// Retrieve a single customer by ID or alias.
    pub async fn get_customer(&self, id_or_alias: String) -> Result<crate::models::Customer> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/customers/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .execute(self.cfg)
            .await
    }

    pub async fn update_customer(
        &self,
        id_or_alias: String,
        customer_update_request: crate::models::CustomerUpdateRequest,
    ) -> Result<crate::models::Customer> {
        crate::request::Request::new(http1::Method::PUT, "/api/v1/customers/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .with_body_param(customer_update_request)
            .execute(self.cfg)
            .await
    }

    /// No linked entity will be deleted. You need to terminate all active subscriptions before archiving a customer, or the call will fail.
    pub async fn archive_customer(&self, id_or_alias: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/api/v1/customers/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Partially update a customer. Only provided fields will be updated.
    pub async fn patch_customer(
        &self,
        id_or_alias: String,
        customer_patch_request: crate::models::CustomerPatchRequest,
    ) -> Result<crate::models::Customer> {
        crate::request::Request::new(http1::Method::PATCH, "/api/v1/customers/{id_or_alias}")
            .with_path_param("id_or_alias", id_or_alias)
            .with_body_param(customer_patch_request)
            .execute(self.cfg)
            .await
    }

    pub async fn get_effective_entitlements(
        &self,
        id_or_alias: String,
    ) -> Result<crate::models::EffectiveEntitlementListResponse> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/customers/{id_or_alias}/entitlements",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .execute(self.cfg)
        .await
    }

    /// Generates a JWT token that grants access to the customer portal.
    /// The token can be used to access invoices, payment methods, and other portal features.
    pub async fn create_portal_token(
        &self,
        id_or_alias: String,
        customer_portal_token_request: crate::models::CustomerPortalTokenRequest,
    ) -> Result<crate::models::CustomerPortalTokenResponse> {
        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/customers/{id_or_alias}/portal-token",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .with_body_param(customer_portal_token_request)
        .execute(self.cfg)
        .await
    }

    pub async fn unarchive_customer(&self, id_or_alias: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/customers/{id_or_alias}/unarchive",
        )
        .with_path_param("id_or_alias", id_or_alias)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
