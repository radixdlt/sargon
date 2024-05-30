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
