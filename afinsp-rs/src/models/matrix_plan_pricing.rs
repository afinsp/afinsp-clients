// this file is @generated
use serde::{Deserialize, Serialize};

use super::matrix_row::MatrixRow;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MatrixPlanPricing {
    pub rates: Vec<MatrixRow>,
}

impl MatrixPlanPricing {
    pub fn new(rates: Vec<MatrixRow>) -> Self {
        Self { rates }
    }
}
