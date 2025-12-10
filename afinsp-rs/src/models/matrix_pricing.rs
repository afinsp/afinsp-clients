// this file is @generated
use serde::{Deserialize, Serialize};

use super::matrix_row::MatrixRow;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MatrixPricing {
    pub rates: Vec<MatrixRow>,
}

impl MatrixPricing {
    pub fn new(rates: Vec<MatrixRow>) -> Self {
        Self { rates }
    }
}
