// this file is @generated
use serde::{Deserialize, Serialize};

use super::{all_components_scope::AllComponentsScope, components_scope::ComponentsScope};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MinimumCommitmentInputScope {
    #[serde(rename = "all_components")]
    AllComponents(AllComponentsScope),
    #[serde(rename = "components")]
    Components(ComponentsScope),
}

impl Default for MinimumCommitmentInputScope {
    fn default() -> Self {
        Self::AllComponents(AllComponentsScope::default())
    }
}
