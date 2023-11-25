use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

use crate::bip32::hd_path_component::HDPathValue;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u32)]
pub enum CAP26KeyKind {
    /// For a key to be used for signing transactions.
    /// The value is the ascii sum of `"TRANSACTION_SIGNING"`
    TransactionSigning = 1460,

    /// For a key to be used for signing authentication..
    /// The value is the ascii sum of `"AUTHENTICATION_SIGNING"`
    AuthenticationSigning = 1678,

    /// For a key to be used for encrypting messages.
    /// The value is the ascii sum of `"MESSAGE_ENCRYPTION"`
    MessageEncryption = 1391,
}

impl Display for CAP26KeyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CAP26KeyKind {
    /// The raw representation of this key kind, an `HDPathValue`.
    pub fn discriminant(&self) -> HDPathValue {
        *self as HDPathValue
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip};

    use crate::cap26::cap26_key_kind::CAP26KeyKind;

    #[test]
    fn discriminant() {
        assert_eq!(CAP26KeyKind::TransactionSigning.discriminant(), 1460);
        assert_eq!(CAP26KeyKind::AuthenticationSigning.discriminant(), 1678);
        assert_eq!(CAP26KeyKind::AuthenticationSigning.discriminant(), 1678);
    }

    #[test]
    fn format() {
        assert_eq!(
            format!("{}", CAP26KeyKind::TransactionSigning),
            "TransactionSigning"
        );
        assert_eq!(
            format!("{}", CAP26KeyKind::AuthenticationSigning),
            "AuthenticationSigning"
        );
        assert_eq!(
            format!("{}", CAP26KeyKind::MessageEncryption),
            "MessageEncryption"
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&CAP26KeyKind::TransactionSigning, json!(1460));
        assert_json_roundtrip(&CAP26KeyKind::TransactionSigning);
    }
}
