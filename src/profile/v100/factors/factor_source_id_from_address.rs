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
    pub fn to_canonical_string(&self) -> String {
        format!("{}:{}", self.kind.discriminant(), self.body)
    }
}

impl HasPlaceholder for FactorSourceIDFromAddress {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(
            FactorSourceKind::TrustedContact,
            AccountAddress::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(
            FactorSourceKind::TrustedContact,
            AccountAddress::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorSourceIDFromAddress::placeholder(),
            FactorSourceIDFromAddress::placeholder()
        );
        assert_eq!(
            FactorSourceIDFromAddress::placeholder_other(),
            FactorSourceIDFromAddress::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceIDFromAddress::placeholder(),
            FactorSourceIDFromAddress::placeholder_other()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", FactorSourceIDFromAddress::placeholder()),
            "trustedContact:account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", FactorSourceIDFromAddress::placeholder()),
            "trustedContact:account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = FactorSourceIDFromAddress::placeholder();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "trustedContact",
                "body": "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
            }
            "#,
        );
    }
}
