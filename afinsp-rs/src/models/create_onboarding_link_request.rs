// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateOnboardingLinkRequest {
    pub redirect_url: String,
}

impl CreateOnboardingLinkRequest {
    pub fn new(redirect_url: String) -> Self {
        Self { redirect_url }
    }
}
