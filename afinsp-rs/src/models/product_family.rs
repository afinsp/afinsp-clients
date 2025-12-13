// this file is @generated
use serde::{Deserialize, Serialize};

use super::product_family_id::ProductFamilyId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductFamily {
    pub id: ProductFamilyId,

    pub name: String,
}

impl ProductFamily {
    pub fn new(id: ProductFamilyId, name: String) -> Self {
        Self { id, name }
    }
}
