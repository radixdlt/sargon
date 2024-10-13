use crate::prelude::*;
use sargon::SecurityQuestions_NOT_PRODUCTION_READY_FactorSource as InternalSecurityQuestions_NOT_PRODUCTION_READY_FactorSource;

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
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
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

#[uniffi::export]
pub fn new_security_questions_factor_source_sample(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    InternalSecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample().into()
}

#[uniffi::export]
pub fn new_security_questions_factor_source_sample_other(
) -> SecurityQuestions_NOT_PRODUCTION_READY_FactorSource {
    InternalSecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample_other()
        .into()
}

#[uniffi::export]
pub fn new_security_questions_factor_source_by_encrypting_mnemonic(
    mnemonic: Mnemonic,
    with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
) -> Result<SecurityQuestions_NOT_PRODUCTION_READY_FactorSource> {
    InternalSecurityQuestions_NOT_PRODUCTION_READY_FactorSource::new_by_encrypting(
        mnemonic.into_internal(), with.into_identified_vec(),
    ).map_result()
}

#[uniffi::export]
pub fn trim_security_questions_answer(answer: String) -> String {
    let kdf = SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8.into_internal();
    kdf.trim_answer(answer)
}

#[uniffi::export]
pub fn security_questions_factor_source_decrypt(
    factor_source: &SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
) -> Result<Mnemonic> {
    factor_source
        .into_internal()
        .decrypt(with.into_identified_vec())
        .map_result()
}
