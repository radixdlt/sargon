use crate::prelude::*;
use sargon::UnspecifiedResources as InternalUnspecifiedResources;

/// Represents unspecified resources, which can be either none present or
/// may be present with a list of change sources.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum UnspecifiedResources {
    /// There are no unspecified resources present
    NonePresent,

    /// There might be non-zero balances of unspecified resources present
    MayBePresent,
}
