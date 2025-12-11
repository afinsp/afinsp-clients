// this file is @generated
use serde::{Deserialize, Serialize};

/// Never resets — counts all usage since the subscription was activated.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NeverResetPeriod {}

impl NeverResetPeriod {
    pub fn new() -> Self {
        Self {}
    }
}
