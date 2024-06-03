use std::sync::Once;

use crate::prelude::*;

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
