// this file is @generated
use serde::{Deserialize, Serialize};

use super::matrix_dimension::MatrixDimension;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MatrixRow {
    pub dimension1: MatrixDimension,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension2: Option<MatrixDimension>,

    pub per_unit_price: rust_decimal::Decimal,
}

impl MatrixRow {
    pub fn new(dimension1: MatrixDimension, per_unit_price: rust_decimal::Decimal) -> Self {
        Self {
            dimension1,
            dimension2: None,
            per_unit_price,
        }
    }
}
