use crate::prelude::*;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    uniffi::Enum,
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

impl CAP26KeyKind {
    /// The raw representation of this key kind, an `HDPathValue`.
    pub fn discriminant(&self) -> HDPathValue {
        *self as HDPathValue
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn discriminant() {
        assert_eq!(CAP26KeyKind::TransactionSigning.discriminant(), 1460);
        assert_eq!(CAP26KeyKind::AuthenticationSigning.discriminant(), 1678);
        assert_eq!(CAP26KeyKind::AuthenticationSigning.discriminant(), 1678);
    }

    #[test]
    fn display() {
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
    fn debug() {
        assert_eq!(
            format!("{:?}", CAP26KeyKind::TransactionSigning),
            "TransactionSigning"
        );
        assert_eq!(
            format!("{:?}", CAP26KeyKind::AuthenticationSigning),
            "AuthenticationSigning"
        );
        assert_eq!(
            format!("{:?}", CAP26KeyKind::MessageEncryption),
            "MessageEncryption"
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &CAP26KeyKind::TransactionSigning,
            json!(1460),
        );
        assert_json_roundtrip(&CAP26KeyKind::TransactionSigning);
    }
}
