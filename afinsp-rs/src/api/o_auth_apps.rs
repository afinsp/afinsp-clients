// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

pub struct OAuthApps<'a> {
    cfg: &'a Configuration,
}

impl<'a> OAuthApps<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List all OAuth applications registered for this platform.
    pub async fn list_oauth_apps(&self) -> Result<crate::models::OAuthAppsResponse> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/oauth-apps")
            .execute(self.cfg)
            .await
    }

    /// Register a new OAuth application. Returns the app with its client secret
    /// (only shown once).
    pub async fn create_oauth_app(
        &self,
        create_o_auth_app_request: crate::models::CreateOAuthAppRequest,
    ) -> Result<crate::models::OAuthAppWithSecret> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/oauth-apps")
            .with_body_param(create_o_auth_app_request)
            .execute(self.cfg)
            .await
    }

    /// Retrieve an OAuth application by ID.
    pub async fn get_oauth_app(&self, id: String) -> Result<crate::models::OAuthApp> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/oauth-apps/{id}")
            .with_path_param("id", id)
            .execute(self.cfg)
            .await
    }

    /// Delete an OAuth application and revoke all associated tokens.
    pub async fn delete_oauth_app(&self, id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/api/v1/oauth-apps/{id}")
            .with_path_param("id", id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Generate a new client secret for an OAuth app. The old secret is
    /// immediately invalidated.
    pub async fn rotate_client_secret(&self, id: String) -> Result<crate::models::RotatedSecret> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/oauth-apps/{id}/rotate")
            .with_path_param("id", id)
            .execute(self.cfg)
            .await
    }
}
