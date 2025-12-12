// this file is @generated
use serde::{Deserialize, Serialize};

use super::online_methods_config::OnlineMethodsConfig;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OnlinePaymentMethodConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OnlineMethodsConfig>,
}

impl OnlinePaymentMethodConfig {
    pub fn new() -> Self {
        Self { config: None }
    }
}
