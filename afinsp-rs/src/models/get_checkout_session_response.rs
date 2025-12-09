// this file is @generated
use serde::{Deserialize, Serialize};

use super::checkout_session::CheckoutSession;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GetCheckoutSessionResponse {
    pub session: CheckoutSession,
}

impl GetCheckoutSessionResponse {
    pub fn new(session: CheckoutSession) -> Self {
        Self { session }
    }
}
