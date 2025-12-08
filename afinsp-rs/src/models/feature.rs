// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    entitlement::Entitlement, entitlement_product_ref::EntitlementProductRef,
    feature_id::FeatureId, feature_status::FeatureStatus, feature_type::FeatureType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Feature {
    /// Unique key used to reference this feature in your code. Cannot be changed after creation.
    pub code: String,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<Entitlement>,

    pub feature_type: FeatureType,

    pub id: FeatureId,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<EntitlementProductRef>,

    pub status: FeatureStatus,
}

impl Feature {
    pub fn new(
        code: String,
        created_at: String,
        feature_type: FeatureType,
        id: FeatureId,
        name: String,
        status: FeatureStatus,
    ) -> Self {
        Self {
            code,
            created_at,
            description: None,
            entitlement: None,
            feature_type,
            id,
            name,
            product: None,
            status,
        }
    }
}
