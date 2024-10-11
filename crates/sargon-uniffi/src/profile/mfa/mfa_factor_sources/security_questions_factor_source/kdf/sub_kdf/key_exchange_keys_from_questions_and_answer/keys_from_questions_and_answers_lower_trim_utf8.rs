use crate::prelude::*;
use sargon::SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 as InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A Key Derivation Scheme which lowercases, trims and ut8f encodes answers.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8;

impl From<InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8> for SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {
    fn from(_: InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8) -> Self {
        Self
    }
}

impl Into<InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8> for SecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {
    fn into(self) -> InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8 {
        InternalSecurityQuestions_NOT_PRODUCTION_READY_KeyExchangeKeysFromQandAsLowerTrimUtf8
    }
}
