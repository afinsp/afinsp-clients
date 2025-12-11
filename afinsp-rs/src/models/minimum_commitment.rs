// this file is @generated
use serde::{Deserialize, Serialize};

use super::minimum_commitment_scope::MinimumCommitmentScope;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MinimumCommitment {
    /// Decimal string in the plan currency, e.g. "100.00".
    pub amount: String,

    pub scope: MinimumCommitmentScope,
}

impl MinimumCommitment {
    pub fn new(amount: String, scope: MinimumCommitmentScope) -> Self {
        Self { amount, scope }
    }
}
