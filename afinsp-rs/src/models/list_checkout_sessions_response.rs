// this file is @generated
use serde::{Deserialize, Serialize};

use super::checkout_session::CheckoutSession;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListCheckoutSessionsResponse {
    pub sessions: Vec<CheckoutSession>,
}

impl ListCheckoutSessionsResponse {
    pub fn new(sessions: Vec<CheckoutSession>) -> Self {
        Self { sessions }
    }
}
