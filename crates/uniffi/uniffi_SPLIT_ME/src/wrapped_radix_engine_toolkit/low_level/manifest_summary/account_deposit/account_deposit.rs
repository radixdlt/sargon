use crate::prelude::*;
use sargon::AccountDeposits as InternalAccountDeposits;

/// Represents an account deposit, which includes specified and unspecified resources.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct AccountDeposits {
    specified_resources: Vec<SimpleResourceBounds>,
    unspecified_resources: UnspecifiedResources,
}
