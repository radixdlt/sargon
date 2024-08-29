use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_security_questions_factor_source_sample(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample()
}

#[uniffi::export]
pub fn new_security_questions_factor_source_sample_other(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample_other()
}

#[uniffi::export]
pub fn new_security_questions_factor_source_by_encrypting_mnemonic(
    mnemonic: Mnemonic,
    with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
) -> Result<SecurityQuestions_NOT_PRODUCTION_READY_FactorSource> {
    SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::new_by_encrypting(
        mnemonic, with,
    )
}

#[uniffi::export]
pub fn trim_security_questions_answer(answer: String) -> String {
    let kdf = SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8;
    kdf.trim_answer(answer)
}

#[uniffi::export]
pub fn security_questions_factor_source_decrypt(
    factor_source: &SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
) -> Result<Mnemonic> {
    factor_source.decrypt(with)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestions_NOT_PRODUCTION_READY_FactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_security_questions_factor_source_sample(),
                new_security_questions_factor_source_sample_other(),
                // duplicates should get removed
                new_security_questions_factor_source_sample(),
                new_security_questions_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn roundtrip() {
        let mnemonic = Mnemonic::sample_security_questions();
        let qas = Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = new_security_questions_factor_source_by_encrypting_mnemonic(
            mnemonic.clone(),
            qas.clone(),
        )
        .unwrap();
        let decrypted =
            security_questions_factor_source_decrypt(&sut, qas).unwrap();
        assert_eq!(decrypted, mnemonic);
    }
}
