use crate::prelude::*;
use sargon::EmailAddress as InternalEmailAddress;

/// An email address.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
#[derive(Clone, PartialEq, Hash, Eq, InternalConversion, uniffi::Record)]
pub struct EmailAddress {
    pub email: String,
}

#[uniffi::export]
pub fn new_email_address_sample() -> EmailAddress {
    InternalEmailAddress::sample().into()
}

#[uniffi::export]
pub fn new_email_address_sample_other() -> EmailAddress {
    InternalEmailAddress::sample_other().into()
}
