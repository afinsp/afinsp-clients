// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct PlansListPlansOptions {
    pub product_family_id: Option<ProductFamilyId>,

    /// Search by plan name
    pub search: Option<String>,

    /// Filter by plan status (can be repeated)
    pub status: Option<Vec<PlanStatusEnum>>,

    /// Filter by plan type (can be repeated)
    pub plan_type: Option<Vec<PlanTypeEnum>>,

    /// Sort order. Format: `column.direction`. Allowed columns: `name`, `status`, `plan_type`, `created_at`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

#[derive(Default)]
pub struct PlansGetPlanDetailsOptions {
    /// Filter by version: "draft", a version number, or omitted for active
    pub version: Option<String>,
}

#[derive(Default)]
pub struct PlansListPlanVersionsOptions {
    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct Plans<'a> {
    cfg: &'a Configuration,
}

impl<'a> Plans<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_plan_version_entitlements(
        &self,
        plan_version_id: String,
    ) -> Result<crate::models::ResolvedEntitlementListResponse> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/plan-versions/{plan_version_id}/entitlements",
        )
        .with_path_param("plan_version_id", plan_version_id)
        .execute(self.cfg)
        .await
    }

    pub async fn list_plans(
        &self,
        options: Option<PlansListPlansOptions>,
    ) -> Result<crate::models::PlanListResponse> {
        let PlansListPlansOptions {
            product_family_id,
            search,
            status,
            plan_type,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/plans")
            .with_optional_query_param("product_family_id", product_family_id)
            .with_optional_query_param("search", search)
            .with_optional_exploded_query_param("status", status)
            .with_optional_exploded_query_param("plan_type", plan_type)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    /// Create a new plan with components and pricing. Set `status` to `ACTIVE` to
    /// publish immediately, or `DRAFT` to stage for review.
    pub async fn create_plan(
        &self,
        create_plan_request: crate::models::CreatePlanRequest,
    ) -> Result<crate::models::Plan> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/plans")
            .with_body_param(create_plan_request)
            .execute(self.cfg)
            .await
    }

    pub async fn set_plan_minimum(
        &self,
        plan_version_id: String,
        minimum_commitment: crate::models::MinimumCommitment,
    ) -> Result<crate::models::MinimumCommitment> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/plans/versions/{plan_version_id}/minimum",
        )
        .with_path_param("plan_version_id", plan_version_id)
        .with_body_param(minimum_commitment)
        .execute(self.cfg)
        .await
    }

    pub async fn delete_plan_minimum(&self, plan_version_id: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/plans/versions/{plan_version_id}/minimum",
        )
        .with_path_param("plan_version_id", plan_version_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Retrieve a specific plan. Use `?version=draft` for the draft version,
    /// `?version=2` for a specific version number, or omit for the active version.
    pub async fn get_plan_details(
        &self,
        plan_id: String,
        options: Option<PlansGetPlanDetailsOptions>,
    ) -> Result<crate::models::Plan> {
        let PlansGetPlanDetailsOptions { version } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", plan_id)
            .with_optional_query_param("version", version)
            .execute(self.cfg)
            .await
    }

    /// Full replacement of a plan's version. On a draft plan, updates in-place.
    /// On a published plan, creates a new version. Set `status` to `DRAFT` to
    /// stage as a new draft without publishing.
    pub async fn replace_plan(
        &self,
        plan_id: String,
        replace_plan_request: crate::models::ReplacePlanRequest,
    ) -> Result<crate::models::Plan> {
        crate::request::Request::new(http1::Method::PUT, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", plan_id)
            .with_body_param(replace_plan_request)
            .execute(self.cfg)
            .await
    }

    /// Partially update plan-level fields (name, description, self_service_rank).
    /// Does not modify version-level configuration or components.
    pub async fn patch_plan(
        &self,
        plan_id: String,
        patch_plan_request: crate::models::PatchPlanRequest,
    ) -> Result<crate::models::Plan> {
        crate::request::Request::new(http1::Method::PATCH, "/api/v1/plans/{plan_id}")
            .with_path_param("plan_id", plan_id)
            .with_body_param(patch_plan_request)
            .execute(self.cfg)
            .await
    }

    pub async fn archive_plan(&self, plan_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/plans/{plan_id}/archive")
            .with_path_param("plan_id", plan_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Publishes the current draft version, making it the active version.
    pub async fn publish_plan(&self, plan_id: String) -> Result<crate::models::Plan> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/plans/{plan_id}/publish")
            .with_path_param("plan_id", plan_id)
            .execute(self.cfg)
            .await
    }

    pub async fn unarchive_plan(&self, plan_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/plans/{plan_id}/unarchive")
            .with_path_param("plan_id", plan_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    pub async fn list_plan_versions(
        &self,
        plan_id: String,
        options: Option<PlansListPlanVersionsOptions>,
    ) -> Result<crate::models::PlanVersionListResponse> {
        let PlansListPlanVersionsOptions { page, per_page } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/plans/{plan_id}/versions")
            .with_path_param("plan_id", plan_id)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }
}
