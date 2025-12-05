// this file is @generated
use serde::{Deserialize, Serialize};

use super::connected_account::ConnectedAccount;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAccountsResponse {
    pub data: Vec<ConnectedAccount>,
}

impl ConnectedAccountsResponse {
    pub fn new(data: Vec<ConnectedAccount>) -> Self {
        Self { data }
    }
}
