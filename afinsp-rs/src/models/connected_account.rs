// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    connected_account_id::ConnectedAccountId, connection_status::ConnectionStatus,
    connection_type::ConnectionType, country_code::CountryCode, customer_id::CustomerId,
    onboarding_mode::OnboardingMode, organization_id::OrganizationId, tenant_id::TenantId,
};

/// A connected account (relationship between platform and connected org)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_organization_id: Option<OrganizationId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_tenant_id: Option<TenantId>,

    pub connection_type: ConnectionType,

    pub created_at: String,

    pub id: ConnectedAccountId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_completed_at: Option<String>,

    pub onboarding_mode: OnboardingMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_country: Option<CountryCode>,

    /// Email of the user being invited (express flow only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_email: Option<String>,

    /// Name of the organization to be created (express flow only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_organization_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_customer_id: Option<CustomerId>,

    pub platform_organization_id: OrganizationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,

    pub status: ConnectionStatus,
}

impl ConnectedAccount {
    pub fn new(
        connection_type: ConnectionType,
        created_at: String,
        id: ConnectedAccountId,
        onboarding_mode: OnboardingMode,
        platform_organization_id: OrganizationId,
        status: ConnectionStatus,
    ) -> Self {
        Self {
            connected_organization_id: None,
            connected_tenant_id: None,
            connection_type,
            created_at,
            id,
            metadata: None,
            onboarding_completed_at: None,
            onboarding_mode,
            pending_country: None,
            pending_email: None,
            pending_organization_name: None,
            platform_customer_id: None,
            platform_organization_id,
            revoked_at: None,
            status,
        }
    }
}
