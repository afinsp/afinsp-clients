// this file is @generated
use serde::{Deserialize, Serialize};

use super::online_method_config::OnlineMethodConfig;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OnlineMethodsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<OnlineMethodConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_debit: Option<OnlineMethodConfig>,
}

impl OnlineMethodsConfig {
    pub fn new() -> Self {
        Self {
            card: None,
            direct_debit: None,
        }
    }
}
