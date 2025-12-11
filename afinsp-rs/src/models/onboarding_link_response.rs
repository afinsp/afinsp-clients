// this file is @generated
use serde::{Deserialize, Serialize};

/// Result of creating an onboarding link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OnboardingLinkResponse {
    pub expires_at: String,

    pub url: String,
}

impl OnboardingLinkResponse {
    pub fn new(expires_at: String, url: String) -> Self {
        Self { expires_at, url }
    }
}
