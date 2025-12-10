// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MatrixDimension {
    pub key: String,

    pub value: String,
}

impl MatrixDimension {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
