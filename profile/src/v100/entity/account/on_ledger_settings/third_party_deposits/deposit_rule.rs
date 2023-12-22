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
    /// The account accepts **all** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account.
    AcceptKnown,
    /// The account accepts **known** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account. By known we mean assets this account has received in the past.
    AcceptAll,
    /// The account denies **all** assets by default, except for exceptions (if any) which might in fact deposit/be deposited into this account.
    DenyAll,
}

impl Default for DepositRule {
    /// By default an account accepts all.
    fn default() -> Self {
        Self::AcceptAll
    }
}

impl DepositRule {
    /// Human readable representation of the rule.
    pub fn discriminant(&self) -> String {
        format!("{}", self)
    }
}

impl Display for DepositRule {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip};
    use serde_json::json;

    use super::DepositRule;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(&DepositRule::AcceptAll, json!("acceptAll"));
        assert_json_roundtrip(&DepositRule::AcceptAll);
    }

    #[test]
    fn inequality() {
        assert_ne!(DepositRule::AcceptAll, DepositRule::DenyAll);
        assert_ne!(DepositRule::DenyAll, DepositRule::AcceptKnown);
        assert_ne!(DepositRule::AcceptAll, DepositRule::AcceptKnown);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", DepositRule::AcceptAll), "AcceptAll");
        assert_eq!(format!("{}", DepositRule::AcceptKnown), "AcceptKnown");

        // `discriminant` uses Display
        assert_eq!(DepositRule::DenyAll.discriminant(), "DenyAll");
    }
}
