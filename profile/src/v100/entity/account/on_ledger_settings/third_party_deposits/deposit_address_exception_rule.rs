use crate::prelude::*;

/// The exception kind for deposit address
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
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum DepositAddressExceptionRule {
    /// A resource can always be deposited in to the account by third-parties
    Allow,
    /// A resource can never be deposited in to the account by third-parties
    Deny,
}

impl DepositAddressExceptionRule {
    pub fn discriminant(&self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for DepositAddressExceptionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(&DepositAddressExceptionRule::Deny, json!("deny"));
        assert_json_roundtrip(&DepositAddressExceptionRule::Deny);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", DepositAddressExceptionRule::Deny), "Deny");

        // `discriminant` uses Display
        assert_eq!(DepositAddressExceptionRule::Allow.discriminant(), "Allow");
    }
}
