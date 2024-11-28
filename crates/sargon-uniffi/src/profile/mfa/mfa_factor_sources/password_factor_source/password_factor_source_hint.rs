use crate::prelude::*;
use sargon::PasswordFactorSourceHint as InternalPasswordFactorSourceHint;

/// Properties describing a PasswordFactorSource to help user disambiguate between
/// it and another one.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PasswordFactorSourceHint {
    pub display_name: DisplayName,
}
