use crate::prelude::*;

use crypto::keys::x25519::SecretKey as X25519PrivateKey;
use hkdf::Hkdf;
use k256::sha2::Sha256;

// impl From<Exactly32Bytes> for X25519PrivateKey {
//     fn from(value: Exactly32Bytes) -> Self {
//         Self::from_bytes(*value.bytes())
//     }
// }

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A Key Derivation Scheme which lowercases, trims and ut8f encodes answers.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8;

impl HasSampleValues for SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {
    fn sample() -> Self {
        Self
    }

    fn sample_other() -> Self {
        Self
    }
}

impl Default
    for SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8
{
    fn default() -> Self {
        Self
    }
}

pub(crate) const SECURITY_QUESTIONS_TRIMMED_CHARS: &[char] = &[
    ' ',  // whitespace
    '\t', // whitespace
    '\n', // whitespace
    '.', // Rationale: Might be natural for some to end answers with a dot, but at a later point in time might be omitted.
    '!', // Rationale: Same as dot
    '?', // Rationale: Same as dot (also strange for an answer to a question to contain a question mark)
    '\'', // Rationale: Feels like an unnecessary risk for differences, sometimes some might omit apostrophe (U+0027)
    '\"', // Rationale: Same as apostrophe (this is "Quotation Mark" (U+0022))
    '‘', // Rationale: Same as apostrophe (this is "Left Single Quotation Mark" (U+2018))
    '’', // Rationale: Same as apostrophe (this is "Right Single Quotation Mark" (U+2019))
    '＇', // Rationale: Same as apostrophe (this is "Full Width Apostrophe" (U+FF07))
];

impl SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {
    pub fn trim_answer(&self, answer: impl AsRef<str>) -> String {
        let mut answer = answer.as_ref().to_lowercase();
        answer.retain(|c| !SECURITY_QUESTIONS_TRIMMED_CHARS.contains(&c));
        answer
    }

    fn bytes_from_answer(&self, answer: impl AsRef<str>) -> Result<Vec<u8>> {
        let answer = answer.as_ref();
        if answer.is_empty() {
            return Err(CommonError::AnswersToSecurityQuestionsCannotBeEmpty);
        }

        let trimmed = self.trim_answer(answer);

        Ok(trimmed.as_bytes().to_owned())
    }

    fn bytes_from_question(&self, question: impl AsRef<str>) -> Vec<u8> {
      question.as_ref().as_bytes().to_owned()
    }
}

impl SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {

    /// ```ignore
    /// let mut attempt 0;
    /// let FORBIDDEN = whitespace.union(delimiters);
    /// let ikm = answer.remove(FORBIDDEN).lowercase();
    /// let info = question.utf8();
    /// let hkdf = Hkdf<SHA256>::new(salt, ikm);
    /// let okm = hkdf.expand(info);
    /// ```
    pub fn derive_key_exchange_key_from_question_and_answer(
        &self,
        question_and_answer: &Security_NOT_PRODUCTION_READY_QuestionAndAnswer,
    ) -> Result<X25519PrivateKey> {
        // Input Key Material: the answer, the most secret.
        let ikm =  self.bytes_from_answer(&question_and_answer.answer)?;

        // We use `question` as info so that two same answers give different
        // output for two different questions, silly example might be:
        // Q1: "Name of best childhood teddy" - A1: "Björn"
        // Q2: "Name of first boy/girl you kissed?" A2: "Björn"
        // Here A1 == A2, but we don't want their keys to be the same, so using 
        // question as `info` => different keys.
        let info = self.bytes_from_question(&question_and_answer.question);

        let hkdf = Hkdf::<Sha256>::new(None, &ikm);
        let mut okm = [0u8; 32];
        hkdf.expand(&info, &mut okm).unwrap();
        Ok(X25519PrivateKey::from_bytes(okm))
    }


}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8;

    #[test]
    fn apa() {
        let sut = SUT::default();
        let non_trimmed = "FoO\nB.a\tR ' ! FiZz ? ‘ B ’ u＇ZZ";
        let trimmed = sut.trim_answer(non_trimmed);
        assert_eq!(trimmed, "foobarfizzbuzz".to_owned())
    }
}
