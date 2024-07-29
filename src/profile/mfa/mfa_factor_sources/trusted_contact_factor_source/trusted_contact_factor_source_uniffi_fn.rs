use crate::prelude::*;

#[uniffi::export]
pub fn new_trusted_contact_factor_source_sample() -> TrustedContactFactorSource
{
    TrustedContactFactorSource::sample()
}

#[uniffi::export]
pub fn new_trusted_contact_factor_source_sample_other(
) -> TrustedContactFactorSource {
    TrustedContactFactorSource::sample_other()
}

#[uniffi::export]
fn new_trusted_contact_factor_source_from_address_and_contact(
    account_address: AccountAddress,
    contact: TrustedContactFactorSourceContact,
) -> TrustedContactFactorSource {
    TrustedContactFactorSource::new(account_address, contact)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TrustedContactFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_trusted_contact_factor_source_sample(),
                new_trusted_contact_factor_source_sample_other(),
                // duplicates should get removed
                new_trusted_contact_factor_source_sample(),
                new_trusted_contact_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new() {
        assert_eq!(
            new_trusted_contact_factor_source_from_address_and_contact(
                AccountAddress::sample_mainnet(),
                TrustedContactFactorSourceContact::sample()
            )
            .factor_source_id(),
            SUT::sample().factor_source_id()
        )
    }
}
