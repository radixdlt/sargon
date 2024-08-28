use std::sync::Once;

use crate::prelude::*;

#[uniffi::export]
pub fn new_trusted_contact_factor_source_contact_sample(
) -> TrustedContactFactorSourceContact {
    TrustedContactFactorSourceContact::sample()
}

#[uniffi::export]
pub fn new_trusted_contact_factor_source_contact_sample_other(
) -> TrustedContactFactorSourceContact {
    TrustedContactFactorSourceContact::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TrustedContactFactorSourceContact;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_trusted_contact_factor_source_contact_sample(),
                new_trusted_contact_factor_source_contact_sample_other(),
                // duplicates should get removed
                new_trusted_contact_factor_source_contact_sample(),
                new_trusted_contact_factor_source_contact_sample_other(),
            ])
            .len(),
            2
        );
    }
}
