use crate::prelude::*;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
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

impl HasSampleValues for FactorSourceIDFromAddress {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(FactorSourceKind::TrustedContact, AccountAddress::sample())
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(
            FactorSourceKind::TrustedContact,
            AccountAddress::sample_other(),
        )
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
            "trustedContact:account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", FactorSourceIDFromAddress::sample()),
            "trustedContact:account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
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
                "body": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
            }
            "#,
        );
    }
}
