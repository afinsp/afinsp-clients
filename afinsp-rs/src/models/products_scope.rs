// this file is @generated
use serde::{Deserialize, Serialize};

/// Only lines for the listed products count. A product is the identity shared by plan
/// components, overrides and ad-hoc extras, so a subscription's billed set is matched uniformly.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductsScope {
    pub product_ids: Vec<String>,
}

impl ProductsScope {
    pub fn new(product_ids: Vec<String>) -> Self {
        Self { product_ids }
    }
}
