use crate::prelude::*;

use aes_gcm::{
    aead::{generic_array::sequence::Concat, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use hkdf::Hkdf;
use k256::sha2::Sha256;

use crypto::keys::x25519::PublicKey as X25519PublicKey;
use crypto::keys::x25519::SecretKey as X25519PrivateKey;

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

    pub fn new_by_encrypting(
        mnemonic: Mnemonic,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Self {
        todo!()
    }

    pub fn decrypt(
        &self,
        with: Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    ) -> Result<Mnemonic> {
        self.sealed_mnemonic.decrypt(with)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityQuestions_NOT_PRODUCTION_READY_FactorSource;

    #[test]
    fn json_then_decrypt() {
        let json = r#"
        {
            "common" :  {
                "addedOn" : "2023-08-17T11:28:24Z",
                "cryptoParameters" :  {
                    "supportedCurves" :  [
                        "curve25519"
                    ],
                    "supportedDerivationPathSchemes" :  [
                        "cap26"
                    ]
                },
                "flags" :  [
                ],
                "lastUsedOn" : "2023-08-17T11:28:24Z"
            },
            "id" :  {
                "body" : "1bd0a4477c874fd2b317896a2ac2af3e4cae51add1c617d7ab6d710f3639ddc1",
                "kind" : "securityQuestions"
            },
            "sealedMnemonic" :  {
                "encryptionScheme" :  {
                    "description" : "AESGCM-256",
                    "version" : 1
                },
                "encryptions" :  [
                    "83f49b1de8bab9037d618e68c2289e40df4a0738b23f5113ada824548cbd32237335a6955c2dd35c0343309ef04b9ebdd18fb37603bdbe5c66278b8294daa42e3d8a37c144c43e8b70ef18033281bfaefe61e1230c81f0a1a744064a653d5c8e3a774ef73a24a22c14f14a0604360c6d96834f611a45c331f0310cf4dda791be36a9a2f4f85cdc8bc308aae47817ddad2d101888a1c08fb0c881dbfa7b7d4cddbb2cade5d4d6ef45d82afa74b6cd01c8ab9c286f1df97182e4",
                    "e68a46e279694716bcf8c3b252fee9cfd3f69339e6082bb791778b7d1f9bc3df69b94a109806b29d78ec36da6e750eaede1f800efc95d4fa685a22b5b26e4c5f6afe05bd73a921fb04ff5e78661708092409deb2883c2184a82c4699899125d3bdc23bf8d336fbc773ccb266b82fec7b4dcac5d69a1f7cc77a6992efb66e7a158ae903c237daca1e455c25106ad7b6db82ab8c7a59c2d8b2c200e243d28c8d16d29b95bcc4652a0ef608518290d705989e603e997bc09c3d14",
                    "ddbfe02dd4281445afb25f9ad7f32b09e044d7fb1dec16f178736176bb1768ea85a601e086a2b75079c7159efbb6d8b04c26c4911d0d4f2c0132a0f6781832df31b43292366b317443998e0c7bc1663401d09b3bd252c06cc6c5214cddf20a9d0ea763aafa26d045ef544a5ad48b74bb97d94fbc008655f79564ff8c42f205735ff7f68d3258de7eeec7dc1bc185ed4e207e6f43224719ee8499f23442d53cadd9884d68154c6c9389b8c65c4b25ca94e89233c0487ac96c6a",
                    "b0817c887d863af233166cbc10aa2fc40c9065576dbe38c4f4e1abd477872879961d50055f20272e0da8fc6e36d6eb183c4f54da59fe1b7f3da4b7f2c288a7c50d1eab0c01314f2b80d9a71cddefb94f4cd6671b00ca30ccbfa93335d2511d892f37be190ac013c5e58f20b560aba49103a19bc54de7c416da089db3808215cb8772229a45de76c33066d9c4ddecd2acfd6e283b3078853e7d2ec6d305e6f18e077d35f6ca7d35e17b81fa5d0baeb3915277a46033db0c5f77"
                ],
                "keyDerivationScheme" :  {
                    "description" : "Lowercase-remove-common-separator-chars-utf8-encode",
                    "version" : 1
                },
                "securityQuestions" :  [
                     {
                        "id" : 0,
                        "kind" : "freeform",
                        "question" : "What's the first name of RDX Works Founder",
                        "version" : 1
                    },
                     {
                        "id" : 1,
                        "kind" : "freeform",
                        "question" : "What's the first name of RDX Works CEO",
                        "version" : 1
                    },
                     {
                        "id" : 2,
                        "kind" : "freeform",
                        "question" : "What's the first name of RDX Works CTO",
                        "version" : 1
                    },
                     {
                        "id" : 3,
                        "kind" : "freeform",
                        "question" : "What's the first name of RDX Works CPO",
                        "version" : 1
                    }
                ]
            }
        }
        "#;
        let sut = serde_json::from_str::<SUT>(&json).unwrap();
        // let qas: Vec<SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes> = sut
        //     .sealed_mnemonic
        //     .security_questions
        //     .clone()
        //     .into_iter()
        //     .enumerate()
        //     .map(|(i, q)| {
        //         let answer = match i {
        //             0 => "Dan",
        //             1 => "Piers",
        //             2 => "Russ",
        //             3 => "Mathew", // incorrect, actually "Matt" was used, but one incorrect is ok!
        //             _ => panic!("too many question")
        //         };
        //         SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes::answer_to_question(answer, q)
        //     }).collect::<Result<Vec<_>>>().unwrap();
        // sut.decrypt(qas).unwrap();
    }
}
