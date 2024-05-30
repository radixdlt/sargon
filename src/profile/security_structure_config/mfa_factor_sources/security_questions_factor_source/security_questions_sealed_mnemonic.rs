use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A mnemonic encrypted by answers to security questions
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
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
    pub encryptions: Vec<Exactly185Bytes>, // FIXME: Set?
}

impl SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
    pub fn new_by_encrypting(
        mnemonic: Mnemonic,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
        kdf_scheme: SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme,
        encryption_scheme: EncryptionScheme,
    ) -> Self {
        let questions_and_answers = with;
        let security_questions = questions_and_answers
            .iter()
            .map(|qa| qa.question.clone())
            .collect::<Security_NOT_PRODUCTION_READY_Questions>();

        let plaintext = serde_json::to_vec(&mnemonic)
            .expect("JSON encoding of Mnemonic should never fail.");

        let encryption_keys = kdf_scheme.derive_encryption_keys_from_questions_and_answers(questions_and_answers).expect("TODO validate that answer is non-empty BEFORE passing it here.");

        let encryptions = encryption_keys
            .into_iter()
            .map(|k| {
                encryption_scheme.encrypt(plaintext.clone(), &mut k.clone())
            })
            .map(|vec| {
                Exactly185Bytes::try_from(vec)
                    .expect("Should have been 185 bytes")
            })
            .collect_vec();

        Self {
            security_questions,
            encryptions,
            kdf_scheme,
            encryption_scheme,
        }
    }

    pub fn decrypt(
        &self,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Mnemonic> {
        let answers_to_question = with;

        let decryption_keys = self
            .kdf_scheme
            .derive_encryption_keys_from_questions_and_answers(
                answers_to_question,
            )?;

        for decryption_key in decryption_keys {
            for encrypted_mnemonic in self.encryptions.iter() {
                match self.encryption_scheme.decrypt(
                    encrypted_mnemonic.bytes(),
                    &mut decryption_key.clone(),
                ) {
                    Ok(decrypted) => {
                        return serde_json::from_slice::<Mnemonic>(&decrypted)
                            .map_err(|_| {
                                CommonError::FailedToDecryptSealedMnemonic
                            })
                    }
                    _ => continue,
                }
            }
        }

        // Failure
        Err(CommonError::FailedToDecryptSealedMnemonic)
    }
}
