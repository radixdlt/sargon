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
/// ```no_run
/// let k = 4;
/// assert_eq!(ec_keys.len(), 6);
/// let key_combinations = Vec<Vec<X25519PrivateKey>> = ec_keys.combinations(k);
/// assert_eq!(key_combinations.len(), 15);
/// assert_eq!(key_combinations.iter().all(|v| v.len() == k);
/// ```
///
/// We map the 15 `Vec<X25519PrivateKey>` into `X25519PublicKeys` using `multi_party_ecdh`:
///
/// ```no_run
/// let ecdh_keys: Vec<X25519PublicKey> = key_combinations.iter().map(multi_party_ecdh).collect();
/// assert_eq!(sec_keys.len(), 15);
/// ```
///
/// Where `multi_party_ecdh` is a function taking `Vec<X25519PrivateKey>` as input and
/// returning a `Key<Aes256Gcm>` by doing key exchange between all keys, like so:
///
/// ```no_run
/// fn multi_party_ecdh(
///     private_keys: Vec<X25519PrivateKey>
/// ) -> Result<Key<X25519PublicKey>> {
///     assert!(private_keys.len() >= 2);
///     let (head, tail) = private_keys.split_off(1);
///     tail.fold(head.public_key(), |acc_pub, x_priv| {
///         let raw = x_priv.hkdf_key_agreement(acc_pub);
///         
///     })
///
///     return try rest.reduce(first.publicKey) { publicKey, privateKey in
///         try Curve25519.KeyAgreement.PublicKey(
///             rawRepresentation: privateKey.sharedSecretFromKeyAgreement(with: publicKey)
///         )
///     }
/// }
/// ```
///
/// We form 15 Symmetric Encryption keys from the 15 `X25519PublicKey` by simply
/// mapping the data of the public keys into AesGCM keys:
///
/// ```no_run
/// let sec_keys: Vec<Key<AesGcm>> = ecdh_keys.iter().map(ec2sec).collect()
/// assert_eq!(sec_keys.len(), 15);
/// ```
///
/// We encrypt the mnemonic 15 times, using each symmetric key in `sec_keys`:
///
/// ```no_run
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
    uniffi::Record,
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
    pub sealed_mnemonic: Sealed_NOT_PRODUCTION_READY_Mnemonic,
}

decl_identified_vec_of!(
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An ordered set of [`SecurityQuestion`]s user has selected to use
    /// in their `SecurityQuestionsFactorSource` - a factor they typically
    /// use for the `ConfirmationRole`.
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Security_NOT_PRODUCTION_READY_Questions,
    Security_NOT_PRODUCTION_READY_Question
);

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A mnemonic encrypted by answers to security questions
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Sealed_NOT_PRODUCTION_READY_Mnemonic {
    pub security_questions: Security_NOT_PRODUCTION_READY_Questions,

    /// A versioned key derivation function algorithm used to produce a set
    /// of encryption keys from answers to `securityQuestions`.
    pub key_derivation_scheme:
        SecurityQuestions_NOT_PRODUCTION_READY_KeyDerivationScheme,

    /// The scheme used to encrypt the Security Questions factor source
    /// mnemonic using one combination of answers to questions, one of many.
    pub encryption_scheme: EncryptionScheme,

    /// The N many encryptions of the mnemonic, where N corresponds to the number of derived keys
    /// from the `keyDerivationScheme`
    pub encryptions: Vec<Exactly32Bytes>, // FIXME: Set?
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// The KDF algorithm used to derive the decryption key from a combination of answers to security questions.
///
/// N.B. Not to be confused with the much simpler password based Key Derivation used
/// to encrypt Profile part of manual file export.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum SecurityQuestions_NOT_PRODUCTION_READY_KeyDerivationScheme {
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// First iteration of KDF for SecurityQuestions
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Version1,
}

#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum SecurityQuestionKind {
    Freeform,
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A security question
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_Question {
    pub id: Uuid,    // FIXME: newtype
    pub version: u8, // FIXME: newtype
    pub kind: SecurityQuestionKind,
    pub question: String,
}

impl Identifiable for Security_NOT_PRODUCTION_READY_Question {
    type ID = Uuid; // FIXME: newtype

    /// Return `Element`'s globally unique and stable ID, used to uniquely identify
    /// the `Element` in the `IdentifiedVecOf` collection of elements.
    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl Security_NOT_PRODUCTION_READY_Question {
    pub fn with_details(
        id: Uuid,
        version: u8,
        kind: SecurityQuestionKind,
        question: impl AsRef<str>,
    ) -> Self {
        Self {
            id,
            version,
            kind,
            question: question.as_ref().to_owned(),
        }
    }
    pub fn new(kind: SecurityQuestionKind, question: impl AsRef<str>) -> Self {
        Self::with_details(id(), 1, kind, question)
    }
    pub fn freeform(question: impl AsRef<str>) -> Self {
        Self::new(SecurityQuestionKind::Freeform, question)
    }
}
impl HasSampleValues for Security_NOT_PRODUCTION_READY_Question {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let mut value =
            Self::freeform("What was the make and model of your first car?");
        value.id = Uuid::from_bytes([0x5a; 16]);
        value
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let mut value =
            Self::freeform("What was the first concert you attended?");
        value.id = Uuid::from_bytes([0x50; 16]);
        value
    }
}

impl HasSampleValues for Security_NOT_PRODUCTION_READY_Questions {
    fn sample() -> Self {
        Self::from_iter([
            Security_NOT_PRODUCTION_READY_Question::sample(),
            Security_NOT_PRODUCTION_READY_Question::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([
            Security_NOT_PRODUCTION_READY_Question::with_details(
                Uuid::from_bytes([0x5e; 16]),
                1,
                SecurityQuestionKind::Freeform,
                "In what city did your parents meet?",
            ),
            Security_NOT_PRODUCTION_READY_Question::sample(),
        ])
    }
}
