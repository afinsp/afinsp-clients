// this file is @generated
use serde::{Deserialize, Serialize};

use super::extra_recurring_billing_type_enum::ExtraRecurringBillingTypeEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExtraRecurringFeeStructure {
    pub billing_type: ExtraRecurringBillingTypeEnum,
}

impl ExtraRecurringFeeStructure {
    pub fn new(billing_type: ExtraRecurringBillingTypeEnum) -> Self {
        Self { billing_type }
    }
}
