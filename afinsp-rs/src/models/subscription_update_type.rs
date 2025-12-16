// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Identifies which mutation triggered a `subscription.updated` webhook.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SubscriptionUpdateType {
    #[default]
    #[serde(rename = "activated")]
    Activated,

    #[serde(rename = "trial_ended")]
    TrialEnded,

    #[serde(rename = "billing_configuration_updated")]
    BillingConfigurationUpdated,

    #[serde(rename = "plan_changed")]
    PlanChanged,

    #[serde(rename = "amended")]
    Amended,

    #[serde(rename = "units_changed")]
    UnitsChanged,

    #[serde(rename = "paused")]
    Paused,

    #[serde(rename = "cancellation_scheduled")]
    CancellationScheduled,
}

impl fmt::Display for SubscriptionUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Activated => "activated",
            Self::TrialEnded => "trial_ended",
            Self::BillingConfigurationUpdated => "billing_configuration_updated",
            Self::PlanChanged => "plan_changed",
            Self::Amended => "amended",
            Self::UnitsChanged => "units_changed",
            Self::Paused => "paused",
            Self::CancellationScheduled => "cancellation_scheduled",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SubscriptionUpdateType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
