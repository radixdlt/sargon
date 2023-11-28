use serde::{Deserialize, Serialize};

use crate::v100::address::account_address::AccountAddress;

use super::factor_source_kind::FactorSourceKind;

/// FactorSourceID from a hash.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FactorSourceIDFromAddress {
    pub kind: FactorSourceKind,
    pub body: AccountAddress,
}

impl FactorSourceIDFromAddress {
    pub fn new(kind: FactorSourceKind, body: AccountAddress) -> Self {
        assert!(kind == FactorSourceKind::TrustedContact, "Only supported FactorSourceKind to be used with  FactorSourceIDFromAddress is `trustedContact` at this moment.");
        Self { kind, body }
    }
}

impl FactorSourceIDFromAddress {
    pub fn placeholder() -> Self {
        Self::new(
            FactorSourceKind::TrustedContact,
            AccountAddress::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::FactorSourceIDFromAddress;

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
