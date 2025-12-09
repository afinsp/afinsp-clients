// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    boolean_feature_type::BooleanFeatureType, config_feature_type::ConfigFeatureType,
    metered_feature_type::MeteredFeatureType,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum FeatureType {
    #[serde(rename = "BOOLEAN")]
    Boolean(BooleanFeatureType),
    #[serde(rename = "METERED")]
    Metered(MeteredFeatureType),
    #[serde(rename = "CONFIG")]
    Config(ConfigFeatureType),
}

impl Default for FeatureType {
    fn default() -> Self {
        Self::Boolean(BooleanFeatureType::default())
    }
}
