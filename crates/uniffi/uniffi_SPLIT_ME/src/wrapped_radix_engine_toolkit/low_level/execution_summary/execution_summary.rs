use crate::prelude::*;
use sargon::ExecutionSummary as InternalExecutionSummary;

/// A summary of the execution of the manifest and the information that helps
/// wallets present the contents of a transaction.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ExecutionSummary {
    /// Per account, a list of all token balances that has been withdrawn from that account.
    pub withdrawals: HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Per account, a list of all token balances that has been deposited into that account.
    pub deposits: HashMap<AccountAddress, Vec<ResourceIndicator>>,

    /// Addresses of accounts encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the accounts of all those addresses, which might be multiple
    /// signatures per Account, if MFA has been setup.
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,

    /// Addresses of identities (Personas) encountered in the manifest where privileged
    /// methods were called. The wallets will need to collect signatures
    /// of the identities of all those addresses, which might be multiple
    /// signatures per Persona, if MFA has been setup.
    pub addresses_of_identities_requiring_auth: Vec<IdentityAddress>,

    /// Information on the global entities created in the transaction.
    pub new_entities: NewEntities,

    /// The manifest classification if any. None means that the manifest is non-conforming.
    pub detailed_classification: Option<DetailedManifestClass>,

    /// List of newly created Non-Fungibles during this transaction.
    pub newly_created_non_fungibles: Vec<NonFungibleGlobalId>,

    /// The set of instructions encountered in the manifest that are reserved
    /// and can only be included in the manifest by the wallet itself.
    pub reserved_instructions: Vec<ReservedInstruction>,

    /// The list of the resources of proofs that were presented in the manifest.
    pub presented_proofs: Vec<ResourceSpecifier>,

    /// The set of all the encountered `ManifestEncounteredComponentAddress`es` in the manifest. This is
    /// to be primarily used for the "using dApps" section of the wallet's tx
    /// review screen.
    pub encountered_addresses: Vec<ManifestEncounteredComponentAddress>,

    /// Information on how much fees were contingent and how much were not.
    pub fee_locks: FeeLocks,

    /// Detailed information on the amount of cost units consumed.
    pub fee_summary: FeeSummary,
}
