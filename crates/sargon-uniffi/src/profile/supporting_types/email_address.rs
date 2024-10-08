use crate::prelude::*;
use sargon::EmailAddress as InternalEmailAddress;

/// An email address.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
#[derive(
    Clone,
    PartialEq,
    Hash,
    Eq,
    uniffi::Record,
)]
pub struct EmailAddress {
    pub email: String,
}

impl From<InternalEmailAddress> for EmailAddress {
    fn from(value: InternalEmailAddress) -> Self {
        Self { email: value.0 }
    }
}

impl Into<InternalEmailAddress> for EmailAddress {
    fn into(self) -> InternalEmailAddress {
        InternalEmailAddress(self.email)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EmailAddress;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_email_address_sample(),
                new_email_address_sample_other(),
                // duplicates should get removed
                new_email_address_sample(),
                new_email_address_sample_other(),
            ])
            .len(),
            2
        );
    }
}
