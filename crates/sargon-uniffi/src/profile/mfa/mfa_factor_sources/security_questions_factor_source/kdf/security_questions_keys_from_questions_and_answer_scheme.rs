use crate::prelude::*;
use sargon::SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme as InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFScheme;
use sargon::SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 as InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// The KDF algorithm used to derive the decryption key from a combination of answers to security questions.
///
/// N.B. Not to be confused with the much simpler password based Key Derivation used
/// to encrypt Profile part of manual file export.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme {
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// First iteration of KDF for SecurityQuestions
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Version1(SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1),
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// Version1 of SecurityQuestions KDF, derives encryption keys from security
/// questions and answers, using two "sub-KDFs".
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
    pub kdf_key_exchanges_keys_from_questions_and_answers: SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8,
    pub kdf_encryption_keys_from_key_exchange_keys: SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold,
}

impl From<InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1>
    for SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1
{
    fn from(
        value: InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1,
    ) -> Self {
        Self {
            kdf_key_exchanges_keys_from_questions_and_answers: value
                .kdf_key_exchanges_keys_from_questions_and_answers
                .into(),
            kdf_encryption_keys_from_key_exchange_keys: value
                .kdf_encryption_keys_from_key_exchange_keys
                .into(),
        }
    }
}

impl Into<InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1>
    for SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1
{
    fn into(
        self,
    ) -> InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
        InternalSecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
            kdf_key_exchanges_keys_from_questions_and_answers: self
                .kdf_key_exchanges_keys_from_questions_and_answers
                .into(),
            kdf_encryption_keys_from_key_exchange_keys: self
                .kdf_encryption_keys_from_key_exchange_keys
                .into(),
        }
    }
}
