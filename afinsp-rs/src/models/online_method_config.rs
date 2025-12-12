// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OnlineMethodConfig {
    pub enabled: bool,
}

impl OnlineMethodConfig {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
