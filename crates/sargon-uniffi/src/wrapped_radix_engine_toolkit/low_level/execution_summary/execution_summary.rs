use crate::prelude::*;
use sargon::ExecutionSummary as InternalExecutionSummary;

/// A summary of the execution of the manifest and the information that helps
/// wallets present the contents of a transaction.
#[derive(Clone, Debug, PartialEq, Eq,  uniffi::Record)]
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

    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub detailed_classification: Vec<DetailedManifestClass>,

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

impl From<InternalExecutionSummary> for ExecutionSummary {
    fn from(value: InternalExecutionSummary) -> Self {
        Self {
            withdrawals: value
                .withdrawals
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|x| x.into()).collect()))
                .collect(),
            deposits: value
                .deposits
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|x| x.into()).collect()))
                .collect(),
            addresses_of_accounts_requiring_auth: value
                .addresses_of_accounts_requiring_auth
                .into_iter()
                .map(|x| x.into())
                .collect(),
            addresses_of_identities_requiring_auth: value
                .addresses_of_identities_requiring_auth
                .into_iter()
                .map(|x| x.into())
                .collect(),
            new_entities: value.new_entities.into(),
            detailed_classification: value
                .detailed_classification
                .into_iter()
                .map(|x| x.into())
                .collect(),
            newly_created_non_fungibles: value
                .newly_created_non_fungibles
                .into_iter()
                .map(|x| x.into())
                .collect(),
            reserved_instructions: value
                .reserved_instructions
                .into_iter()
                .map(|x| x.into())
                .collect(),
            presented_proofs: value
                .presented_proofs
                .into_iter()
                .map(|x| x.into())
                .collect(),
            encountered_addresses: value
                .encountered_addresses
                .into_iter()
                .map(|x| x.into())
                .collect(),
            fee_locks: value.fee_locks.into(),
            fee_summary: value.fee_summary.into(),
        }
    }
}

impl Into<InternalExecutionSummary> for ExecutionSummary {
    fn into(self) -> InternalExecutionSummary {
        InternalExecutionSummary {
            withdrawals: self
                .withdrawals
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|x| x.into()).collect()))
                .collect(),
            deposits: self
                .deposits
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_iter().map(|x| x.into()).collect()))
                .collect(),
            addresses_of_accounts_requiring_auth: self
                .addresses_of_accounts_requiring_auth
                .into_iter()
                .map(|x| x.into())
                .collect(),
            addresses_of_identities_requiring_auth: self
                .addresses_of_identities_requiring_auth
                .into_iter()
                .map(|x| x.into())
                .collect(),
            new_entities: self.new_entities.into(),
            detailed_classification: self
                .detailed_classification
                .into_iter()
                .map(|x| x.into())
                .collect(),
            newly_created_non_fungibles: self
                .newly_created_non_fungibles
                .into_iter()
                .map(|x| x.into())
                .collect(),
            reserved_instructions: self
                .reserved_instructions
                .into_iter()
                .map(|x| x.into())
                .collect(),
            presented_proofs: self
                .presented_proofs
                .into_iter()
                .map(|x| x.into())
                .collect(),
            encountered_addresses: self
                .encountered_addresses
                .into_iter()
                .map(|x| x.into())
                .collect(),
            fee_locks: self.fee_locks.into(),
            fee_summary: self.fee_summary.into(),
        }
    }
}