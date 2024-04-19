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

impl DepositRule {
    pub fn from_json_string(json: impl AsRef<str>) -> Result<Self> {
        let json_string = json.as_ref().to_owned();
        let json_value = serde_json::Value::String(json_string.clone());
        serde_json::from_value(json_value).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: json_string.len() as u64,
                type_name: type_name::<Self>(),
            }
        })
    }

    pub fn to_json_string(&self) -> String {
        let value = serde_json::to_value(self).unwrap_or_else(|_| {
            unreachable!(
                "JSON serialization of {} should never fail.",
                type_name::<Self>()
            )
        });
        match value {
            serde_json::Value::String(str) => str.to_owned(),
            _ => unreachable!("never happen"),
        }
    }
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
    fn json_string_roundtrip() {
        let sut = SUT::sample();
        let json_str = sut.to_json_string();
        println!("json verbatim: '{}'", &json_str);
        assert_eq!(SUT::from_json_string(json_str).unwrap(), sut)
    }

    #[test]
    fn from_json_str() {
        assert_eq!(SUT::from_json_string("acceptAll").unwrap(), SUT::AcceptAll);
        assert_eq!(SUT::from_json_string("denyAll").unwrap(), SUT::DenyAll);
        assert_eq!(
            SUT::from_json_string("acceptKnown").unwrap(),
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
