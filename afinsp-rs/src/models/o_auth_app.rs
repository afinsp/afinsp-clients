// this file is @generated
use serde::{Deserialize, Serialize};

use super::{o_auth_app_id::OAuthAppId, organization_id::OrganizationId};

/// An OAuth application registered by a platform
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OAuthApp {
    pub client_id: String,

    pub client_secret_hint: String,

    pub created_at: String,

    pub id: OAuthAppId,

    pub is_active: bool,

    pub name: String,

    pub organization_id: OrganizationId,

    pub redirect_uris: Vec<String>,

    pub scopes: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl OAuthApp {
    pub fn new(
        client_id: String,
        client_secret_hint: String,
        created_at: String,
        id: OAuthAppId,
        is_active: bool,
        name: String,
        organization_id: OrganizationId,
        redirect_uris: Vec<String>,
        scopes: Vec<String>,
    ) -> Self {
        Self {
            client_id,
            client_secret_hint,
            created_at,
            id,
            is_active,
            name,
            organization_id,
            redirect_uris,
            scopes,
            updated_at: None,
        }
    }
}
