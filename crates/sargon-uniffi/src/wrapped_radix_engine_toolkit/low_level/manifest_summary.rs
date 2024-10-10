use crate::prelude::*;
use sargon::ManifestSummary as InternalManifestSummary;

/// A summary of the manifest
#[derive(Clone, Debug, PartialEq, Eq,  uniffi::Record)]
pub struct ManifestSummary {
    /// Addresses of accounts withdrawn from in the manifest.
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,

    /// Addresses of accounts deposited into in the manifest.
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the accounts of all those addresses, which might be multiple
    /// signatures per Account, if MFA has been setup.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the identities of all those addresses, which might be multiple
    /// signatures per Persona, if MFA has been setup.
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl From<InternalManifestSummary> for ManifestSummary {
    fn from(value: InternalManifestSummary) -> Self {
        Self {
            addresses_of_accounts_withdrawn_from: value
                .addresses_of_accounts_withdrawn_from
                .into_vec(),
            addresses_of_accounts_deposited_into: value.addresses_of_accounts_deposited_into.into_vec(),
            addresses_of_accounts_requiring_auth: value.addresses_of_accounts_requiring_auth.into_vec(),
            addresses_of_personas_requiring_auth: value.addresses_of_personas_requiring_auth.into_vec(),
        }
    }
}

impl Into<InternalManifestSummary> for ManifestSummary {
    fn into(self) -> InternalManifestSummary {
        InternalManifestSummary {
            addresses_of_accounts_withdrawn_from: self.addresses_of_accounts_withdrawn_from.into(),
            addresses_of_accounts_deposited_into: self.addresses_of_accounts_deposited_into.into(),
            addresses_of_accounts_requiring_auth: self.addresses_of_accounts_requiring_auth.into(),
            addresses_of_personas_requiring_auth: self.addresses_of_personas_requiring_auth.into(),
        }
    }
}

