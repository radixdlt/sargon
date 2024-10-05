use crate::prelude::*;

/// An email address.
///
/// Current implementation does not validate the email address other than it
/// cannot be empty (in the future we might add some simple validation).
#[derive(
    Clone,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{email}")]
#[debug("{email}")]
#[serde(transparent)]
pub struct EmailAddress {
    pub email: String,
}

impl Identifiable for EmailAddress {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.email.clone()
    }
}

#[uniffi::export]
pub fn new_email_address_sample() -> EmailAddress {
    EmailAddress::sample()
}

#[uniffi::export]
pub fn new_email_address_sample_other() -> EmailAddress {
    EmailAddress::sample_other()
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
