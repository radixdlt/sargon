use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// The KDF algorithm used to derive the decryption key from a combination of answers to security questions.
///
/// N.B. Not to be confused with the much simpler password based Key Derivation used
/// to encrypt Profile part of manual file export.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme {
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// First iteration of KDF for SecurityQuestions
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Version1(SecurityQuestions_NOT_PRODUCTION_READY_KDFSchemeVersion1),
}