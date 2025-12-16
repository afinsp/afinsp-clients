// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SubscriptionStatusEnum {
    #[default]
    #[serde(rename = "PENDING_ACTIVATION")]
    PendingActivation,

    #[serde(rename = "PENDING_CHARGE")]
    PendingCharge,

    #[serde(rename = "TRIAL_ACTIVE")]
    TrialActive,

    #[serde(rename = "ACTIVE")]
    Active,

    #[serde(rename = "TRIAL_EXPIRED")]
    TrialExpired,

    #[serde(rename = "PAUSED")]
    Paused,

    #[serde(rename = "SUSPENDED")]
    Suspended,

    #[serde(rename = "CANCELLED")]
    Cancelled,

    #[serde(rename = "ABORTED")]
    Aborted,

    #[serde(rename = "COMPLETED")]
    Completed,

    #[serde(rename = "SUPERSEDED")]
    Superseded,

    #[serde(rename = "ERRORED")]
    Errored,
}

impl fmt::Display for SubscriptionStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::PendingActivation => "PENDING_ACTIVATION",
            Self::PendingCharge => "PENDING_CHARGE",
            Self::TrialActive => "TRIAL_ACTIVE",
            Self::Active => "ACTIVE",
            Self::TrialExpired => "TRIAL_EXPIRED",
            Self::Paused => "PAUSED",
            Self::Suspended => "SUSPENDED",
            Self::Cancelled => "CANCELLED",
            Self::Aborted => "ABORTED",
            Self::Completed => "COMPLETED",
            Self::Superseded => "SUPERSEDED",
            Self::Errored => "ERRORED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SubscriptionStatusEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
