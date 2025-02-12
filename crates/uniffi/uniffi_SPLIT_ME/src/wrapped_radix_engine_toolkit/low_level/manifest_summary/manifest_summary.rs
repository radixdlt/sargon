use crate::prelude::*;
use sargon::ManifestSummary as InternalManifestSummary;

/// A summary of the manifest
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ManifestSummary {
    /// The withdrawals done in the manifest.
    pub account_withdrawals: HashMap<AccountAddress, Vec<AccountWithdraw>>,

    /// The deposits done in the manifest.
    pub account_deposits: HashMap<AccountAddress, AccountDeposits>,

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

    /// The various classifications that this manifest matched against. Note
    /// that an empty set means that the manifest is non-conforming.
    pub classification: Vec<ManifestClass>,
}

use sargon::RetManifestClass as InternalManifestClass;

/// The classification process classifies manifests into classes. The following
/// are the classes that the Radix Engine Toolkit supports.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum ManifestClass {
    /// A subintent manifest which satisfies the general rules allowed for in
    /// general transactions and that includes a [`YieldToParent`] instruction.
    ///
    /// [`YieldToParent`]: radix_transactions::manifest::YieldToParent
    GeneralSubintent,
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    General,
    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer,
    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution,
    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption,
    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake,
    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake,
    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaimXrd,
    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate,
}
