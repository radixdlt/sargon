use encryption::EncryptionScheme;
use prelude::fixture_profile_model;

use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A mnemonic "sealed" by "security questions" (personal questions).
///
/// The user select P personal questions from a set of Q predefined questions,
/// then answers them. The user will be able to "open" (decrypt) the "sealed"
/// (encrypted) mnemonic by providing at least P-1 correct answers to the P,
/// questions, that is to say, she is allowed to input one incorrect answer.
/// This is important since it makes this factor source kind more convenient
/// to use, especially if a significant time has passed between user answering
/// the questions for the first and the second time.
///
/// In order to make it possible for user to input one incorrect answer, we need
/// to encrypt the mnemonic with many keys, keys being formed from many combinations
/// of question-answer-based input. To do this we use a function `qna2bin`
/// (question and answer to binary), to deterministically form Curve25519 key pairs,
/// P many (one per question/answer pair), and then we combine these keys using
/// ECDH (key exchange) to form composite (symmetric) encryption keys using P-1
/// many Ed25519 keys per composite encryption key.
///
/// E.g. User selects 6 questions out of 20:
/// Q1: "What was the make and models of your first car?"
/// Q2: "In which town and which year did your parents meet?"
/// Q3: "What was the name of your first stuffed animal?"
/// Q4: "What was the name of the boy or the girl you first kissed?"
/// Q5: "What was the first exam you failed?"
/// Q6: "What is the middle name of your youngest child?"
///
/// She answers them, forming
/// `let qa: Vec<(Questions, Answer)> = [(q_0, a_0), (q_1, a_1), .., (q_5, a_5)]`
/// question, answer pairs.
///
/// The answers will be "normalized", trying to make it easier for user to provide
/// the same used answer later in time, we can do this by for example removing
/// whitespace, delimiters and apostrophes, and lowercase all chars.
///
/// We form 6 binary inputs, call them `bins` using function `qna2bin` taking a
/// question/answer pair as input and outputting 32 bytes.
/// `let bins: Vec<Exactly32Bytes> = qa.iter().map(qna2bin).collect()`
///
/// We form 6 X25519PrivateKey meant for Diffie-Hellman key exchange from `bin`
/// `let ec_keys: Vec<X25519PrivateKey> = bins.iter().map(bin2key).collect()`
///
/// We form ["6 choose 4" ("P choose P-1")][choose] = 15 many combinations
/// (e.g. using [`combinations method from itertools`](itertools))
///
/// ```ignore
/// let k = 4;
/// assert_eq!(ec_keys.len(), 6);
/// let key_combinations = Vec<Vec<X25519PrivateKey>> = ec_keys.combinations(k);
/// assert_eq!(key_combinations.len(), 15);
/// assert_eq!(key_combinations.iter().all(|v| v.len() == k);
/// ```
///
/// We map the 15 `Vec<X25519PrivateKey>` into `X25519PublicKeys` using `multi_party_ecdh`:
///
/// ```ignore
/// let ecdh_keys: Vec<X25519PublicKey> = key_combinations.iter().map(multi_party_ecdh).collect();
/// assert_eq!(sec_keys.len(), 15);
/// ```
///
/// Where `multi_party_ecdh` is a function taking `Vec<X25519PrivateKey>` as input and
/// returning a `Key<Aes256Gcm>` by doing key exchange between all keys, like so:
///
/// ```ignore
/// fn key_exchange_between_more_than_two_keys(
///     &self,
///     between: Vec<&X25519PrivateKey>,
/// ) -> X25519PublicKey {
///     let mut private_keys = between.clone();
///     assert!(private_keys.len() > 2);
///     let tail = private_keys.split_off(1);
///     let head = private_keys.into_iter().last().unwrap();
///
///     tail.into_iter().fold(head.public_key(), |acc_pub, x_priv| {
///         let shared_secret = x_priv.diffie_hellman(&acc_pub);
///         X25519PublicKey::from_bytes(shared_secret.to_bytes())
///     })
/// }
/// ```
///
/// We form 15 Symmetric Encryption keys from the 15 `X25519PublicKey` by simply
/// mapping the data of the public keys into AesGCM keys:
///
/// ```ignore
/// let sec_keys: Vec<Key<AesGcm>> = ecdh_keys.iter().map(ec2sec).collect()
/// assert_eq!(sec_keys.len(), 15);
/// ```
///
/// We encrypt the mnemonic 15 times, using each symmetric key in `sec_keys`:
///
/// ```ignore
/// let encryptions: Vec<AesGcmSealedBox> = sec_keys.iter().map(|x| x.enc)
/// assert_eq!(encryptions.len(), 15);
/// ```
///
/// Decryption is then the reverse process, trying to decrypt any of the 15
/// encrypted mnemonics with any of the 15 symmetric (de)encryption keys we
/// re-calculate from the answers user gives at this later point in time.
///
/// Author / Inventor: Alexander Cyon (alex.cyon@gmail.com) in the year 2022.
///
/// ❗️ NOT PRODUCTION READY YET ❗️
///
/// [choose]: https://en.wikipedia.org/wiki/Combination
/// [itertools]: https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.combinations
///
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{id}")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// The sealed mnemonic containing multiple different encryptions of a
    /// mnemonic encrypted by different encryptions keys, being various combinations
    /// of questions and answers derived keys, allowing for only 4 out of 6 answers
    /// to be correct.
    pub sealed_mnemonic: SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic,
}

impl SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    pub fn with_details(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        sealed_mnemonic: SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic,
    ) -> Self {
        Self {
            id,
            common,
            sealed_mnemonic,
        }
    }

    pub fn new_by_encrypting_with_schemes(
        mnemonic: Mnemonic,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
        kdf_scheme: SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme,
        encryption_scheme: EncryptionScheme,
    ) -> Result<Self> {
        let questions_and_answers = with;
        let id = FactorSourceIDFromHash::new_for_security_questions(
            &MnemonicWithPassphrase::new(mnemonic.clone()),
        );
        let sealed_mnemonic = SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic::new_by_encrypting(mnemonic, questions_and_answers, kdf_scheme, encryption_scheme)?;
        let common = FactorSourceCommon::new_babylon();

        Ok(Self {
            id,
            sealed_mnemonic,
            common,
        })
    }

    pub fn new_by_encrypting(
        mnemonic: Mnemonic,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Self> {
        let questions_and_answers = with;
        Self::new_by_encrypting_with_schemes(
            mnemonic,
            questions_and_answers,
            SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme::default(),
            EncryptionScheme::default(),
        )
    }

    pub fn decrypt(
        &self,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Mnemonic> {
        let questions_and_answers = with;
        self.sealed_mnemonic.decrypt(questions_and_answers)
    }
}

impl HasSampleValues for SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    fn sample() -> Self {
        let json =
            fixture_profile_model!("security_questions_factor_source_sample");
        let sut = serde_json::from_str::<Self>(json).unwrap();
        let decrypted =
            sut.decrypt(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
            )
            .unwrap();
        assert_eq!(decrypted, Mnemonic::sample_security_questions());
        sut
    }

    fn sample_other() -> Self {
        let json = fixture_profile_model!(
            "security_questions_factor_source_sample_other"
        );
        let sut = serde_json::from_str::<Self>(json).unwrap();
        let decrypted = sut
            .decrypt(
                Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other(
                ),
            )
            .unwrap();
        assert_eq!(decrypted, Mnemonic::sample_security_questions_other());
        sut
    }
}

impl From<SecurityQuestions_NOT_PRODUCTION_READY_FactorSource>
    for FactorSource
{
    fn from(
        value: SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    ) -> Self {
        FactorSource::SecurityQuestions { value }
    }
}

impl TryFrom<FactorSource>
    for SecurityQuestions_NOT_PRODUCTION_READY_FactorSource
{
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        value.clone().into_security_questions().map_err(|_| {
            CommonError::InvalidFactorSourceKind {
                bad_value: value.factor_source_kind().to_string(),
            }
        })
    }
}
impl IsFactorSource for SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::SecurityQuestions
    }
}
impl BaseBaseIsFactorSource
    for SecurityQuestions_NOT_PRODUCTION_READY_FactorSource
{
    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }

    fn name(&self) -> String {
        let ids = self
            .sealed_mnemonic
            .security_questions
            .items()
            .into_iter()
            .map(|q| q.id())
            .map(|id| format!("#{:?}", id))
            .join(", ");
        format!("Questions: {}", ids)
    }

    fn set_name(&mut self, _updated: String) {
        unreachable!("SecurityQuestions cannot be renamed");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestions_NOT_PRODUCTION_READY_FactorSource;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_factor_source() {
        let sut = SUT::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(SUT::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: "device".to_owned()
            })
        );
    }

    #[test]
    fn roundtrip_sample_all_answers_correct() {
        let m = Mnemonic::sample_security_questions();
        let qas = Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    impl Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
        fn insert_bad_chars_to_answer(&mut self) {
            let bad: String =
                String::from_iter(SECURITY_QUESTIONS_TRIMMED_CHARS);
            self.answer = format!("{}{}{}", bad, self.answer, bad);
        }
    }

    #[test]
    fn roundtrip_sample_one_incorrect_answer_is_ok() {
        let m = Mnemonic::sample_security_questions();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change to one wrong answer when decrypting
        qas.update_with(0, |qa| qa.answer = "wrong".to_owned());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_two_incorrect_answer_is_ok() {
        let m = Mnemonic::sample_security_questions();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change to two wrong answers when decrypting
        qas.update_with(0, |qa| qa.answer = "wrong".to_owned());
        qas.update_with(3, |qa| qa.answer = "also wrong".to_owned());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_case_does_not_matter() {
        let m = Mnemonic::sample_security_questions();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change all answers to uppercase before decrypting is ok
        qas.update_with(0, |qa| qa.answer = qa.answer.to_uppercase());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_bad_chars_are_trimmed() {
        let m = Mnemonic::sample_security_questions();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Inserting bad chars into answer before decrypting.
        qas.update_with(0, |qa| qa.insert_bad_chars_to_answer());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_other_all_answers_correct() {
        let m = Mnemonic::sample_security_questions_other();
        let qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_other_case_does_not_matter() {
        let m = Mnemonic::sample_security_questions_other();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change all answers to uppercase before decrypting is ok
        qas.update_with(0, |qa| qa.answer = qa.answer.to_uppercase());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn roundtrip_sample_other_one_incorrect_answer_is_ok() {
        let m = Mnemonic::sample_security_questions_other();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change to two wrong answers when decrypting
        qas.update_with(0, |qa| qa.answer = "wrong".to_owned());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn kind() {
        assert_eq!(SUT::kind(), FactorSourceKind::SecurityQuestions);
    }

    #[test]
    fn roundtrip_sample_other_two_incorrect_answer_is_ok() {
        let m = Mnemonic::sample_security_questions_other();
        let mut qas =
            Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample_other();
        let sut = SUT::new_by_encrypting(m.clone(), qas.clone()).unwrap();

        // Change to two wrong answers when decrypting
        qas.update_with(0, |qa| qa.answer = "wrong".to_owned());
        qas.update_with(3, |qa| qa.answer = "also wrong".to_owned());
        let decrypted = sut.decrypt(qas).unwrap();
        assert_eq!(m, decrypted);
    }

    #[test]
    fn test_too_few_questions() {
        let m = Mnemonic::sample();
        type Q = Security_NOT_PRODUCTION_READY_Question;
        let q0 = Q::drivings_instructor();
        let a0 = "a";

        let q1 = Q::child_middle_name();
        let a1 = "d";

        let q2 = Q::math_teacher_highschool();
        let a2 = "b";

        let q3 = Q::first_school();
        let a3 = "c";

        #[allow(clippy::upper_case_acronyms)]
        type QAS = Security_NOT_PRODUCTION_READY_QuestionsAndAnswers;
        #[allow(clippy::upper_case_acronyms)]
        type QA = Security_NOT_PRODUCTION_READY_QuestionAndAnswer;
        let qas = QAS::from_iter([
            QA::new(q0, a0),
            QA::new(q1, a1),
            QA::new(q2, a2),
            QA::new(q3, a3),
        ]);
        let res = SUT::new_by_encrypting(m.clone(), qas.clone());

        assert_eq!(
            res,
            Err(CommonError::InvalidQuestionsAndAnswersCount {
                expected: 6,
                found: 4
            })
        );
    }

    #[test]
    fn name() {
        assert_eq!(SUT::sample().name(), "Questions: #0, #1, #2, #3, #4, #5");
    }

    #[should_panic(expected = "SecurityQuestions cannot be renamed")]
    #[test]
    fn set_name() {
        SUT::sample().set_name("whatever".to_string())
    }
}
