use crate::prelude::*;

use radix_engine_interface::blueprints::account::DefaultDepositRule as ScryptoDefaultDepositRule;

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
    enum_iterator::Sequence,
    derive_more::Display,
    uniffi::Enum,
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

impl From<DepositRule> for ScryptoDefaultDepositRule {
    fn from(value: DepositRule) -> Self {
        match value {
            DepositRule::AcceptKnown => {
                ScryptoDefaultDepositRule::AllowExisting
            }
            DepositRule::AcceptAll => ScryptoDefaultDepositRule::Accept,
            DepositRule::DenyAll => ScryptoDefaultDepositRule::Reject,
        }
    }
}

impl From<ScryptoDefaultDepositRule> for DepositRule {
    fn from(value: ScryptoDefaultDepositRule) -> Self {
        match value {
            ScryptoDefaultDepositRule::Accept => Self::AcceptAll,
            ScryptoDefaultDepositRule::Reject => Self::DenyAll,
            ScryptoDefaultDepositRule::AllowExisting => Self::AcceptKnown,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(
            &DepositRule::AcceptAll,
            json!("acceptAll"),
        );
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
        assert_eq!(format!("{}", DepositRule::DenyAll), "DenyAll");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", DepositRule::AcceptAll), "AcceptAll");
        assert_eq!(format!("{:?}", DepositRule::AcceptKnown), "AcceptKnown");
        assert_eq!(format!("{:?}", DepositRule::DenyAll), "DenyAll");
    }
}
