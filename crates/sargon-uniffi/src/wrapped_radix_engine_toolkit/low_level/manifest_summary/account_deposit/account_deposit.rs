use crate::prelude::*;
use sargon::AccountDeposit as InternalAccountDeposit;

/// Represents an account deposit, which includes specified and unspecified resources.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct AccountDeposit {
    specified_resources: HashMap<ResourceAddress, SimpleResourceBounds>,
    unspecified_resources: UnspecifiedResources,
}