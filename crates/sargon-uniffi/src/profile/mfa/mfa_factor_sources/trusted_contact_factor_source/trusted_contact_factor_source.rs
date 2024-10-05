use crate::prelude::*;

/// A factor source representing a person, company, organization or otherwise
/// entity that the user trusts to help her with recovery, if ever needed.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{contact} {id}")]
pub struct TrustedContactFactorSource {
    /// Unique and stable identifier of this factor source.
    pub id: FactorSourceIDFromAddress,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// The contact information about the contact that is 'trusted'.
    pub contact: TrustedContactFactorSourceContact,
}

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
