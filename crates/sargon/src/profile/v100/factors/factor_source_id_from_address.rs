use crate::prelude::*;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.to_canonical_string())]
#[debug("{}", self.to_canonical_string())]
pub struct FactorSourceIDFromAddress {
    /// The kind of the FactorSource this ID refers to, typically `trustedContact`.
    pub kind: FactorSourceKind,

    /// An account address which the FactorSource this ID refers uses/needs.
    pub body: AccountAddress,
}

impl FactorSourceIDFromAddress {
    pub fn new(kind: FactorSourceKind, body: AccountAddress) -> Self {
        assert!(kind == FactorSourceKind::TrustedContact, "Only supported FactorSourceKind to be used with  FactorSourceIDFromAddress is `trustedContact` at this moment.");
        Self { kind, body }
    }
}

impl FactorSourceIDFromAddress {
    pub fn new_for_trusted_contact(address: AccountAddress) -> Self {
        Self::new(FactorSourceKind::TrustedContact, address)
    }
}

impl FactorSourceIDFromAddress {
    pub fn to_canonical_string(&self) -> String {
        format!("{}:{}", self.kind.discriminant(), self.body)
    }
}

impl FactorSourceIDFromAddress {
    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_frank() -> Self {
        Self::new_for_trusted_contact(AccountAddress::sample_frank())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_judy() -> Self {
        Self::new_for_trusted_contact(AccountAddress::sample_judy())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_contact_friend_oscar() -> Self {
        Self::new_for_trusted_contact(AccountAddress::sample_oscar())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_trusted_entity_radix() -> Self {
        Self::new_for_trusted_contact(AccountAddress::sample_radix())
    }
}

impl HasSampleValues for FactorSourceIDFromAddress {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_trusted_contact_friend_frank()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_trusted_contact_friend_oscar()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorSourceIDFromAddress::sample(),
            FactorSourceIDFromAddress::sample()
        );
        assert_eq!(
            FactorSourceIDFromAddress::sample_other(),
            FactorSourceIDFromAddress::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceIDFromAddress::sample(),
            FactorSourceIDFromAddress::sample_other()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", FactorSourceIDFromAddress::sample()),
            "trustedContact:account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", FactorSourceIDFromAddress::sample()),
            "trustedContact:account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorSourceIDFromAddress::sample();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "trustedContact",
                "body": "account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
            }
            "#,
        );
    }
}
