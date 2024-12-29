use crate::prelude::*;
use sargon::EmailAddress as InternalEmailAddress;

/// An email address.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
#[derive(Clone, PartialEq, Hash, Eq, uniffi::Record)]
pub struct EmailAddress {
    pub email: String,
}

impl EmailAddress {
    pub fn into_internal(&self) -> InternalEmailAddress {
        self.clone().into()
    }
}
impl From<InternalEmailAddress> for EmailAddress {
    fn from(value: InternalEmailAddress) -> Self {
        Self {
            email: value.to_string(),
        }
    }
}
impl From<EmailAddress> for InternalEmailAddress {
    fn from(value: EmailAddress) -> Self {
        Self::new(value.email.as_str()).unwrap()
    }
}

#[uniffi::export]
pub fn new_email_address_sample() -> EmailAddress {
    InternalEmailAddress::sample().into()
}

#[uniffi::export]
pub fn new_email_address_sample_other() -> EmailAddress {
    InternalEmailAddress::sample_other().into()
}
