use crate::prelude::*;
use sargon::CAP26KeyKind as InternalCAP26KeyKind;

#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
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

impl From<InternalCAP26KeyKind> for CAP26KeyKind {
    fn from(value: InternalCAP26KeyKind) -> Self {
        match value {
            InternalCAP26KeyKind::TransactionSigning => Self::TransactionSigning,
            InternalCAP26KeyKind::AuthenticationSigning => Self::AuthenticationSigning,
            InternalCAP26KeyKind::MessageEncryption => Self::MessageEncryption,
        }
    }
}

impl Into<InternalCAP26KeyKind> for CAP26KeyKind {
    fn into(self) -> InternalCAP26KeyKind {
        match self {
            Self::TransactionSigning => InternalCAP26KeyKind::TransactionSigning,
            Self::AuthenticationSigning => InternalCAP26KeyKind::AuthenticationSigning,
            Self::MessageEncryption => InternalCAP26KeyKind::MessageEncryption,
        }
    }
}