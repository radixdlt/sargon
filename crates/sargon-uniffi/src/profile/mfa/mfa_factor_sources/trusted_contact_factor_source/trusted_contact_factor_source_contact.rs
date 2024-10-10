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

