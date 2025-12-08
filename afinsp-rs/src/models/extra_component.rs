// this file is @generated
use serde::{Deserialize, Serialize};

use super::{price_entry::PriceEntry, product_ref::ProductRef};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExtraComponent {
    pub name: String,

    pub price_entry: PriceEntry,

    pub product_ref: ProductRef,
}

impl ExtraComponent {
    pub fn new(name: String, price_entry: PriceEntry, product_ref: ProductRef) -> Self {
        Self {
            name,
            price_entry,
            product_ref,
        }
    }
}
