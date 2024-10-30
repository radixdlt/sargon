use crate::prelude::*;
use sargon::ManifestSummary as InternalManifestSummary;

/// A summary of the manifest
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ManifestSummary {
    /// The withdrawals done in the manifest.
    pub account_withdrawals: HashMap<AccountAddress, Vec<AccountWithdraw>>,

    /// The deposits done in the manifest.
    pub account_deposits: HashMap<AccountAddress, Vec<AccountDeposit>>,

    /// The list of the resources of proofs that were presented in the manifest.
    pub presented_proofs: Vec<ResourceSpecifier>,

    /// Addresses of accounts withdrawn from in the manifest.
    pub addresses_of_accounts_withdrawn_from: Vec<AccountAddress>,

    /// Addresses of accounts deposited into in the manifest.
    pub addresses_of_accounts_deposited_into: Vec<AccountAddress>,

    /// The set of all the global entities encountered in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_entities: Vec<ManifestEncounteredComponentAddress>,

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

    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: Vec<ReservedInstruction>,
}
