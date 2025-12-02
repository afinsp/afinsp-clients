// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct CustomPropertiesListDefinitionsOptions {
    /// Filter to a single entity type.
    pub entity_type: Option<CustomPropertyEntityType>,

    /// Include archived (soft-deleted) definitions. Defaults to false.
    pub include_archived: Option<bool>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct CustomProperties<'a> {
    cfg: &'a Configuration,
}

impl<'a> CustomProperties<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_definitions(
        &self,
        options: Option<CustomPropertiesListDefinitionsOptions>,
    ) -> Result<crate::models::CustomPropertyDefinitionListResponse> {
        let CustomPropertiesListDefinitionsOptions {
            entity_type,
            include_archived,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/custom-property-definitions")
            .with_optional_query_param("entity_type", entity_type)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    pub async fn create_definition(
        &self,
        custom_property_definition_create_request: crate::models::CustomPropertyDefinitionCreateRequest,
    ) -> Result<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/custom-property-definitions")
            .with_body_param(custom_property_definition_create_request)
            .execute(self.cfg)
            .await
    }

    pub async fn get_definition(
        &self,
        id: String,
    ) -> Result<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .execute(self.cfg)
        .await
    }

    pub async fn update_definition(
        &self,
        id: String,
        custom_property_definition_update_request: crate::models::CustomPropertyDefinitionUpdateRequest,
    ) -> Result<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .with_body_param(custom_property_definition_update_request)
        .execute(self.cfg)
        .await
    }

    /// Soft-deletes the definition. Existing property values on entities are preserved; the definition
    /// simply stops being enforced on new writes.
    pub async fn archive_definition(
        &self,
        id: String,
    ) -> Result<crate::models::CustomPropertyDefinition> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/custom-property-definitions/{id}",
        )
        .with_path_param("id", id)
        .execute(self.cfg)
        .await
    }
}
