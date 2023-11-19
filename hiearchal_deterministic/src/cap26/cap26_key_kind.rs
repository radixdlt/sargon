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
        write!(f, "{}", self.description())
    }
}
impl CAP26KeyKind {
    /// The raw representation of this key kind, an `HDPathValue`.
    pub fn discriminant(&self) -> HDPathValue {
        *self as HDPathValue
    }

    fn description(&self) -> String {
        match self {
            Self::TransactionSigning => "TransactionSigning".to_string(),
            Self::AuthenticationSigning => "AuthenticationSigning".to_string(),
            Self::MessageEncryption => "MessageEncryption".to_string(),
        }
    }
}
