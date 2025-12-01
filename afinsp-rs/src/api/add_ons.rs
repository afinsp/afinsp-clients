// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct AddOnsListAddonsOptions {
    pub search: Option<String>,

    pub currency: Option<String>,

    /// Include archived add-ons in the results (default: false)
    pub include_archived: Option<bool>,

    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct AddOns<'a> {
    cfg: &'a Configuration,
}

impl<'a> AddOns<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_addons(
        &self,
        options: Option<AddOnsListAddonsOptions>,
    ) -> Result<crate::models::AddOnListResponse> {
        let AddOnsListAddonsOptions {
            search,
            currency,
            include_archived,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/addons")
            .with_optional_query_param("search", search)
            .with_optional_query_param("currency", currency)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    pub async fn create_addon(
        &self,
        create_add_on_request: crate::models::CreateAddOnRequest,
    ) -> Result<crate::models::AddOn> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/addons")
            .with_body_param(create_add_on_request)
            .execute(self.cfg)
            .await
    }

    pub async fn get_addon(&self, addon_id: String) -> Result<crate::models::AddOn> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/addons/{addon_id}")
            .with_path_param("addon_id", addon_id)
            .execute(self.cfg)
            .await
    }

    pub async fn update_addon(
        &self,
        addon_id: String,
        update_add_on_request: crate::models::UpdateAddOnRequest,
    ) -> Result<crate::models::AddOn> {
        crate::request::Request::new(http1::Method::PATCH, "/api/v1/addons/{addon_id}")
            .with_path_param("addon_id", addon_id)
            .with_body_param(update_add_on_request)
            .execute(self.cfg)
            .await
    }

    pub async fn archive_addon(&self, addon_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/addons/{addon_id}/archive")
            .with_path_param("addon_id", addon_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    pub async fn list_add_on_entitlements(
        &self,
        addon_id: String,
    ) -> Result<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/addons/{addon_id}/entitlements")
            .with_path_param("addon_id", addon_id)
            .execute(self.cfg)
            .await
    }

    pub async fn unarchive_addon(&self, addon_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/addons/{addon_id}/unarchive")
            .with_path_param("addon_id", addon_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }
}
