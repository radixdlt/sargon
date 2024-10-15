use crate::prelude::*;
use sargon::SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic as InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A mnemonic encrypted by answers to security questions
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
    pub security_questions: Security_NOT_PRODUCTION_READY_Questions,

    /// A versioned Key Derivation Function (KDF) algorithm used to produce a set
    /// of Encryption keys from a set of security questions and answers
    pub kdf_scheme: SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme,

    /// The scheme used to encrypt the Security Questions factor source
    /// mnemonic using one combination of answers to questions, one of many.
    pub encryption_scheme: EncryptionScheme,

    /// The N many encryptions of the mnemonic, where N corresponds to the number of derived keys
    /// from the `keyDerivationScheme`
    pub encryptions: Vec<Exactly60Bytes>, // FIXME: Set?
}

impl From<InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic>
    for SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic
{
    fn from(
        value: InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic,
    ) -> Self {
        Self {
            security_questions: value.security_questions.into_vec(),
            kdf_scheme: value.kdf_scheme.into(),
            encryption_scheme: value.encryption_scheme.into(),
            encryptions: value.encryptions.into_vec(),
        }
    }
}

impl Into<InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic>
    for SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic
{
    fn into(
        self,
    ) -> InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
        InternalSecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
            security_questions: self.security_questions.into_internal(),
            kdf_scheme: self.kdf_scheme.into(),
            encryption_scheme: self.encryption_scheme.into(),
            encryptions: self.encryptions.into_internal(),
        }
    }
}
