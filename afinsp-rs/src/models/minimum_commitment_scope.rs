// this file is @generated
use serde::{Deserialize, Serialize};

use super::{all_components_scope::AllComponentsScope, products_scope::ProductsScope};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MinimumCommitmentScope {
    #[serde(rename = "all_components")]
    AllComponents(AllComponentsScope),
    #[serde(rename = "products")]
    Products(ProductsScope),
}

impl Default for MinimumCommitmentScope {
    fn default() -> Self {
        Self::AllComponents(AllComponentsScope::default())
    }
}
