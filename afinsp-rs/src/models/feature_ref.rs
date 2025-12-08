// this file is @generated
use serde::{Deserialize, Serialize};

use super::{entitlement_product_ref::EntitlementProductRef, feature_id::FeatureId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FeatureRef {
    /// Unique key used to reference this feature in your code. Cannot be changed after creation.
    pub code: String,

    pub id: FeatureId,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<EntitlementProductRef>,
}

impl FeatureRef {
    pub fn new(code: String, id: FeatureId, name: String) -> Self {
        Self {
            code,
            id,
            name,
            product: None,
        }
    }
}
