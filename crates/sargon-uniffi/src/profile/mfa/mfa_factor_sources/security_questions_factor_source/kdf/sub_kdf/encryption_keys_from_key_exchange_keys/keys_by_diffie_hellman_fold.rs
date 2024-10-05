use crate::prelude::*;
/// ❗️ NOT PRODUCTION READY YET ❗️
/// A key derivation function which produces Encryption Keys from a set of
/// key exchange keys, by performing Diffie-Hellman key exchange on each
/// Key Exchange Key in a Set, by "folding" from left to right.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_EncryptionKeysByDiffieHellmanFold;
