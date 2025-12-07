// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomTaxRate {
    pub name: String,

    pub rate: String,

    pub tax_code: String,
}

impl CustomTaxRate {
    pub fn new(name: String, rate: String, tax_code: String) -> Self {
        Self {
            name,
            rate,
            tax_code,
        }
    }
}
