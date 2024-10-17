use crate::prelude::*;
use sargon::ManifestSummary as InternalManifestSummary;

/// A summary of the manifest
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
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
