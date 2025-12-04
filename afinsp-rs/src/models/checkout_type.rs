// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CheckoutType {
    #[default]
    #[serde(rename = "SELF_SERVE")]
    SelfServe,

    #[serde(rename = "SUBSCRIPTION_ACTIVATION")]
    SubscriptionActivation,

    #[serde(rename = "PLAN_CHANGE")]
    PlanChange,

    #[serde(rename = "ADDON_PURCHASE")]
    AddonPurchase,
}

impl fmt::Display for CheckoutType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::SelfServe => "SELF_SERVE",
            Self::SubscriptionActivation => "SUBSCRIPTION_ACTIVATION",
            Self::PlanChange => "PLAN_CHANGE",
            Self::AddonPurchase => "ADDON_PURCHASE",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CheckoutType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
