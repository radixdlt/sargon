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
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum DepositAddressExceptionRule {
    /// A resource can always be deposited in to the account by third-parties
    Allow,
    /// A resource can never be deposited in to the account by third-parties
    Deny,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(
            &DepositAddressExceptionRule::Deny,
            json!("deny"),
        );
        assert_json_roundtrip(&DepositAddressExceptionRule::Deny);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", DepositAddressExceptionRule::Deny), "Deny");
        assert_eq!(format!("{}", DepositAddressExceptionRule::Allow), "Allow");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", DepositAddressExceptionRule::Deny), "Deny");
        assert_eq!(
            format!("{:?}", DepositAddressExceptionRule::Allow),
            "Allow"
        );
    }
}
