use crate::prelude::*;

/// Hints about the trusted contact.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    
     uniffi::Record,
)]
pub struct TrustedContactFactorSourceContact {
    /// The email address of the contact that the user trusts
    pub email_address: EmailAddress,
    /// The name of the contact that the user trusts
    pub name: DisplayName,
}

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
