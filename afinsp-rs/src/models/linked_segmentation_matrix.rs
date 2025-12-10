// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkedSegmentationMatrix {
    pub dimension1_key: String,

    pub dimension2_key: String,

    pub values: std::collections::HashMap<String, Vec<String>>,
}

impl LinkedSegmentationMatrix {
    pub fn new(
        dimension1_key: String,
        dimension2_key: String,
        values: std::collections::HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            dimension1_key,
            dimension2_key,
            values,
        }
    }
}
