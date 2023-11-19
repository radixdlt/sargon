use std::fmt::Display;

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

/// The general deposit rule to apply
#[derive(
    Serialize,
    Deserialize,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Sequence,
)]
#[serde(rename_all = "camelCase")]
pub enum DepositRule {
    AcceptAll,
    AcceptKnown,
    DenyAll,
}

impl Default for DepositRule {
    fn default() -> Self {
        Self::AcceptAll
    }
}

impl DepositRule {
    pub fn discriminant(&self) -> String {
        format!("{}", self)
    }
}

impl Display for DepositRule {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip};

    use super::DepositRule;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(&DepositRule::AcceptAll, json!("acceptAll"));
        assert_json_roundtrip(&DepositRule::AcceptAll);
    }
}
