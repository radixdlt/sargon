use crate::prelude::*;
use sargon::TrustedContactFactorSourceContact as InternalTrustedContactFactorSourceContact;

/// Hints about the trusted contact.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct TrustedContactFactorSourceContact {
    /// The email address of the contact that the user trusts
    pub email_address: EmailAddress,
    /// The name of the contact that the user trusts
    pub name: DisplayName,
}

#[uniffi::export]
pub fn new_trusted_contact_factor_source_contact_sample(
) -> TrustedContactFactorSourceContact {
    InternalTrustedContactFactorSourceContact::sample().into()
}

#[uniffi::export]
pub fn new_trusted_contact_factor_source_contact_sample_other(
) -> TrustedContactFactorSourceContact {
    InternalTrustedContactFactorSourceContact::sample_other().into()
}
