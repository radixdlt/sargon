use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::v100::AccountAddress;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use super::factor_source_kind::FactorSourceKind;

/// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
pub struct FactorSourceIDFromAddress {
    /// The kind of the FactorSource this ID refers to, typically `trustedContact`.
    kind: FactorSourceKind,

    /// An account address which the FactorSource this ID refers uses/needs.
    body: AccountAddress,
}

impl FactorSourceIDFromAddress {
    pub fn new(kind: FactorSourceKind, body: AccountAddress) -> Self {
        assert!(kind == FactorSourceKind::TrustedContact, "Only supported FactorSourceKind to be used with  FactorSourceIDFromAddress is `trustedContact` at this moment.");
        Self { kind, body }
    }
}

#[cfg(any(test, feature = "placeholder"))]
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
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::FactorSourceIDFromAddress;

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
