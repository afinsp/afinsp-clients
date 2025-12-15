// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    slot_downgrade_policy_enum::SlotDowngradePolicyEnum,
    slot_upgrade_policy_enum::SlotUpgradePolicyEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlotFeeStructure {
    pub downgrade_policy: SlotDowngradePolicyEnum,

    pub slot_unit_name: String,

    pub upgrade_policy: SlotUpgradePolicyEnum,
}

impl SlotFeeStructure {
    pub fn new(
        downgrade_policy: SlotDowngradePolicyEnum,
        slot_unit_name: String,
        upgrade_policy: SlotUpgradePolicyEnum,
    ) -> Self {
        Self {
            downgrade_policy,
            slot_unit_name,
            upgrade_policy,
        }
    }
}
