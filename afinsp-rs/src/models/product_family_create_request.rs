// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductFamilyCreateRequest {
    pub name: String,
}

impl ProductFamilyCreateRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
