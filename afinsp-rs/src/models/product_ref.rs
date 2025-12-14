// this file is @generated
use serde::{Deserialize, Serialize};

use super::{existing_product_ref::ExistingProductRef, new_product_ref::NewProductRef};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ProductRef {
    #[serde(rename = "EXISTING")]
    Existing(ExistingProductRef),
    #[serde(rename = "NEW")]
    New(NewProductRef),
}

impl Default for ProductRef {
    fn default() -> Self {
        Self::Existing(ExistingProductRef::default())
    }
}
