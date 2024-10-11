use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// The KDF algorithm used to derive the decryption key from a combination of answers to security questions.
///
/// N.B. Not to be confused with the much simpler password based Key Derivation used
/// to encrypt Profile part of manual file export.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme {
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// First iteration of KDF for SecurityQuestions
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Version1(SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1),
}

impl Default for SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme {
    fn default() -> Self {
        Self::Version1(
            SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1::default(),
        )
    }
}

impl IsSecurityQuestionsKDFScheme
    for SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme
{
    fn derive_encryption_keys_from_questions_and_answers(
        &self,
        questions_and_answers: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Vec<EncryptionKey>> {
        match self {
            Self::Version1(kdf) => kdf
                .derive_encryption_keys_from_questions_and_answers(
                    questions_and_answers,
                ),
        }
    }
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// Version1 of SecurityQuestions KDF, derives encryption keys from security
/// questions and answers, using two "sub-KDFs".
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
    pub kdf_key_exchanges_keys_from_questions_and_answers: SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8,
    pub kdf_encryption_keys_from_key_exchange_keys: SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold,
}

impl HasSampleValues for SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
    fn sample() -> Self {
        Self {
            kdf_key_exchanges_keys_from_questions_and_answers: SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8::sample(),
            kdf_encryption_keys_from_key_exchange_keys: SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            kdf_key_exchanges_keys_from_questions_and_answers: SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8::sample_other(),
            kdf_encryption_keys_from_key_exchange_keys: SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold::sample_other(),
        }
    }
}

impl Default for SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1 {
    fn default() -> Self {
        Self {
            kdf_key_exchanges_keys_from_questions_and_answers: SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8,
            kdf_encryption_keys_from_key_exchange_keys: SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold
        }
    }
}

impl IsSecurityQuestionsKDFScheme
    for SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1
{
    fn derive_encryption_keys_from_questions_and_answers(
        &self,
        questions_and_answers: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Vec<EncryptionKey>> {
        let kdf_kek = &self.kdf_key_exchanges_keys_from_questions_and_answers;
        let kdf_enc = &self.kdf_encryption_keys_from_key_exchange_keys;

        let kek = questions_and_answers
            .iter()
            .map(|qa| {
                kdf_kek.derive_key_exchange_key_from_question_and_answer(&qa)
            })
            .collect::<Result<_>>()?;

        Ok(kdf_enc.derive_encryption_keys_from(kek))
    }
}

impl HasSampleValues for SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme {
    fn sample() -> Self {
        Self::Version1(SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1::sample())
    }

    fn sample_other() -> Self {
        Self::Version1(SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1::sample_other())
    }
}
