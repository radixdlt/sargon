use crate::prelude::*;
use sargon::CAP26KeyKind as InternalCAP26KeyKind;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
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
