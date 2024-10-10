use crate::prelude::*;

/// A factor source representing a person, company, organization or otherwise
/// entity that the user trusts to help her with recovery, if ever needed.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    
     uniffi::Record,
)]
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

