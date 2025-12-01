// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

pub struct Connect<'a> {
    cfg: &'a Configuration,
}

impl<'a> Connect<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List all connected accounts for this platform.
    pub async fn list_connected_accounts(
        &self,
    ) -> Result<crate::models::ConnectedAccountsResponse> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/connected-accounts")
            .execute(self.cfg)
            .await
    }

    /// Create a new connected account (Express flow). Returns the account
    /// and an onboarding link for the user to complete setup.
    pub async fn create_connected_account(
        &self,
        create_connected_account_request: crate::models::CreateConnectedAccountRequest,
    ) -> Result<crate::models::ConnectedAccount> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/connected-accounts")
            .with_body_param(create_connected_account_request)
            .execute(self.cfg)
            .await
    }

    /// Retrieve a connected account by ID.
    pub async fn get_connected_account(
        &self,
        id: String,
    ) -> Result<crate::models::ConnectedAccount> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/connected-accounts/{id}")
            .with_path_param("id", id)
            .execute(self.cfg)
            .await
    }

    /// Revoke a connected account. All associated tokens are invalidated.
    pub async fn disconnect_account(&self, id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/api/v1/connected-accounts/{id}")
            .with_path_param("id", id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Generate a new onboarding link for a connected account. Any existing
    /// unused link is invalidated. The link expires after a configured duration.
    pub async fn create_onboarding_link(
        &self,
        id: String,
        create_onboarding_link_request: crate::models::CreateOnboardingLinkRequest,
    ) -> Result<crate::models::OnboardingLinkResponse> {
        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/connected-accounts/{id}/onboarding",
        )
        .with_path_param("id", id)
        .with_body_param(create_onboarding_link_request)
        .execute(self.cfg)
        .await
    }
}
