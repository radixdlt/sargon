use crate::prelude::*;

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

impl HasSampleValues for DepositRule {
    fn sample() -> Self {
        Self::AcceptKnown
    }

    fn sample_other() -> Self {
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
    use core_utils::prelude::DeserializeStr;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DepositRule;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(
            &SUT::AcceptAll,
            json!("acceptAll"),
        );
        assert_json_roundtrip(&SUT::AcceptAll);
    }

    #[test]
    fn from_json_str() {
        assert_eq!(
            "acceptAll".deserialize::<DepositRule>().unwrap(),
            SUT::AcceptAll
        );
        assert_eq!(
            "denyAll".deserialize::<DepositRule>().unwrap(),
            SUT::DenyAll
        );
        assert_eq!(
            "acceptKnown".deserialize::<DepositRule>().unwrap(),
            SUT::AcceptKnown
        )
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::AcceptAll), "AcceptAll");
        assert_eq!(format!("{}", SUT::AcceptKnown), "AcceptKnown");
        assert_eq!(format!("{}", SUT::DenyAll), "DenyAll");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::AcceptAll), "AcceptAll");
        assert_eq!(format!("{:?}", SUT::AcceptKnown), "AcceptKnown");
        assert_eq!(format!("{:?}", SUT::DenyAll), "DenyAll");
    }

    #[test]
    fn scrypto_roundtrip() {
        let roundtrip = |s: SUT| {
            assert_eq!(SUT::from(ScryptoDefaultDepositRule::from(s)), s)
        };
        roundtrip(SUT::AcceptKnown);
        roundtrip(SUT::AcceptAll);
        roundtrip(SUT::DenyAll);
    }
}
