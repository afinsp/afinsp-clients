// this file is @generated
use serde::{Deserialize, Serialize};

use super::minimum_commitment_input_scope::MinimumCommitmentInputScope;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MinimumCommitmentInput {
    /// Decimal string in the plan currency.
    pub amount: String,

    pub scope: MinimumCommitmentInputScope,
}

impl MinimumCommitmentInput {
    pub fn new(amount: String, scope: MinimumCommitmentInputScope) -> Self {
        Self { amount, scope }
    }
}
