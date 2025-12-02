// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

pub struct OAuth<'a> {
    cfg: &'a Configuration,
}

impl<'a> OAuth<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Token introspection endpoint (RFC 7662). Requires client credentials
    /// via HTTP Basic auth.
    pub async fn introspect_endpoint(
        &self,
        introspection_request: crate::models::IntrospectionRequest,
    ) -> Result<crate::models::TokenIntrospectionResponse> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/oauth/introspect")
            .with_form_body_param(introspection_request)
            .execute(self.cfg)
            .await
    }

    /// Token revocation endpoint (RFC 7009). Always returns 200 per spec.
    /// Requires client credentials via HTTP Basic auth.
    pub async fn revoke_endpoint(
        &self,
        revocation_request: crate::models::RevocationRequest,
    ) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/oauth/revoke")
            .with_form_body_param(revocation_request)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// OAuth 2.0 token endpoint. Supports two grant types:
    /// - `authorization_code`: Exchange an authorization code for tokens
    /// - `refresh_token`: Refresh an access token
    ///
    /// Authenticate via HTTP Basic auth (`client_id:client_secret`) or body parameters.
    pub async fn token_endpoint(
        &self,
        token_request: crate::models::TokenRequest,
    ) -> Result<crate::models::TokenResponse> {
        crate::request::Request::new(http1::Method::POST, "/api/v1/oauth/token")
            .with_form_body_param(token_request)
            .execute(self.cfg)
            .await
    }
}
