// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    entitlement_id::EntitlementId, entitlement_value::EntitlementValue, feature_id::FeatureId,
};

/// A raw entitlement row attached to one entity (feature, plan version, add-on, or subscription).
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Entitlement {
    pub created_at: String,

    pub feature_id: FeatureId,

    pub id: EntitlementId,

    pub updated_at: String,

    pub value: EntitlementValue,
}

impl Entitlement {
    pub fn new(
        created_at: String,
        feature_id: FeatureId,
        id: EntitlementId,
        updated_at: String,
        value: EntitlementValue,
    ) -> Self {
        Self {
            created_at,
            feature_id,
            id,
            updated_at,
            value,
        }
    }
}
