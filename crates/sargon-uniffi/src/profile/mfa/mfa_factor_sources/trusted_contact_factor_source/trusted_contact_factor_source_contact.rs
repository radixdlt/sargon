use crate::prelude::*;
use sargon::TrustedContactFactorSourceContact as InternalTrustedContactFactorSourceContact;

/// Hints about the trusted contact.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TrustedContactFactorSourceContact {
    /// The email address of the contact that the user trusts
    pub email_address: EmailAddress,
    /// The name of the contact that the user trusts
    pub name: DisplayName,
}

impl From<InternalTrustedContactFactorSourceContact>
    for TrustedContactFactorSourceContact
{
    fn from(value: InternalTrustedContactFactorSourceContact) -> Self {
        Self {
            email_address: value.email_address.into(),
            name: value.name.into(),
        }
    }
}

impl Into<InternalTrustedContactFactorSourceContact>
    for TrustedContactFactorSourceContact
{
    fn into(self) -> InternalTrustedContactFactorSourceContact {
        InternalTrustedContactFactorSourceContact {
            email_address: self.email_address.into(),
            name: self.name.into(),
        }
    }
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
