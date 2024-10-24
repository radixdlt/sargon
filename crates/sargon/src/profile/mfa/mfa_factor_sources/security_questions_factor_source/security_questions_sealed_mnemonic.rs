use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A mnemonic encrypted by answers to security questions
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
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
    pub encryptions: Vec<Exactly60Bytes>, // FIXME: Set?
}

impl SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
    pub const QUESTION_COUNT: usize = 6;
    pub fn new_by_encrypting(
        mnemonic: Mnemonic,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
        kdf_scheme: SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme,
        encryption_scheme: EncryptionScheme,
    ) -> Result<Self> {
        let questions_and_answers = with;
        if questions_and_answers.len() != Self::QUESTION_COUNT {
            return Err(CommonError::InvalidQuestionsAndAnswersCount {
                expected: Self::QUESTION_COUNT as u16,
                found: questions_and_answers.len() as u16,
            });
        }
        let security_questions = questions_and_answers
            .iter()
            .map(|qa| qa.question.clone())
            .collect::<Security_NOT_PRODUCTION_READY_Questions>();

        let mnemonic_entropy = Exactly32Bytes::try_from(mnemonic.to_entropy().as_ref()).expect("SecurityQuestionsFactorSource mnemonics SHOULD ALWAYS be 32 bytes of entropy.");

        let encryption_keys = kdf_scheme.derive_encryption_keys_from_questions_and_answers(questions_and_answers).expect("TODO validate that answer is non-empty BEFORE passing it here.");

        let encryptions = encryption_keys
            .into_iter()
            .map(|k| {
                encryption_scheme.encrypt(mnemonic_entropy, &mut k.clone())
            })
            .map(|vec| {
                Exactly60Bytes::try_from(vec)
                    .expect("Should have been 60 bytes")
            })
            .collect_vec();

        Ok(Self {
            security_questions,
            encryptions,
            kdf_scheme,
            encryption_scheme,
        })
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
                match self
                    .encryption_scheme
                    .decrypt(
                        encrypted_mnemonic.bytes(),
                        &mut decryption_key.clone(),
                    )
                    .and_then(Exactly32Bytes::try_from)
                {
                    Ok(decrypted) => {
                        return Ok(Mnemonic::from_32bytes_entropy(decrypted))
                    }
                    _ => continue,
                }
            }
        }

        // Failure
        Err(CommonError::FailedToDecryptSealedMnemonic)
    }
}

impl HasSampleValues for SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic {
    fn sample() -> Self {
        let mnemonic = Mnemonic::sample();
        let questions_and_answers =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let kdf_scheme =
            SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme::default();
        let encryption_scheme = EncryptionScheme::default();
        Self::new_by_encrypting(
            mnemonic,
            questions_and_answers,
            kdf_scheme,
            encryption_scheme,
        )
        .expect("Should have been able to create a sample")
    }

    fn sample_other() -> Self {
        let mnemonic = Mnemonic::sample();
        let questions_and_answers =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other();
        let kdf_scheme =
            SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme::default();
        let encryption_scheme = EncryptionScheme::default();
        Self::new_by_encrypting(
            mnemonic,
            questions_and_answers,
            kdf_scheme,
            encryption_scheme,
        )
        .expect("Should have been able to create a sample")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic;

    #[test]
    fn throws_if_incorrect_count() {
        let too_few = Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::just(
            Security_NOT_PRODUCTION_READY_QuestionAndAnswer::sample(),
        );
        let res = SUT::new_by_encrypting(
            Mnemonic::sample(),
            too_few,
            SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme::default(),
            EncryptionScheme::default(),
        );
        assert_eq!(
            res,
            Err(CommonError::InvalidQuestionsAndAnswersCount {
                expected: 6,
                found: 1
            })
        );
    }
}
