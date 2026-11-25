use crate::prelude::*;
use sargon::DetailedManifestClass as InternalDetailedManifestClass;

/// The execution summary process not only determines the class of the manifest,
/// but also includes additional information about this class that the wallet
/// requires to display to the user.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum DetailedManifestClass {
    /// A general manifest that involves any amount of arbitrary components
    /// and packages where nothing more concrete can be said about the manifest
    /// and its nature.
    ///
    /// No additional information is required beyond what the execution summary
    /// will provide.
    General,

    /// A manifest of a 1-to-1 transfer to a one-to-many transfer of resources.
    Transfer {
        /// When `true`, then this is a one-to-one transfer and the wallet can
        /// regard this as a "simple transfer" and communicate this information
        /// to the ledger hardware wallet. Otherwise, if `false`, then this is
        /// not a one-to-one transfer.
        is_one_to_one_transfer: bool,
    },

    /// A manifest where XRD is claimed from one or more validators.
    ValidatorClaim {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The claims observed in the transaction
        validator_claims: Vec<TrackedValidatorClaim>,
    },

    /// A manifest where XRD is staked to one or more validators.
    ValidatorStake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,
        /// The stake observed in the transaction
        validator_stakes: Vec<TrackedValidatorStake>,
    },

    /// A manifest where XRD is unstaked from one or more validators.
    ValidatorUnstake {
        /// The addresses of validators in the transaction
        validator_addresses: Vec<ValidatorAddress>,

        /// The data associated with the various claim NFTs
        claims_non_fungible_data: HashMap<NonFungibleGlobalId, UnstakeData>,
    },

    /// A manifest that updated the deposit settings of the account.
    AccountDepositSettingsUpdate {
        /// Updates to the resource preferences of the account deposit settings.
        /// account_address -> (resource_address -> Update<new_preference>)
        resource_preferences_updates: HashMap<
            AccountAddress,
            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
        >,
        /// Changes to the account's deposit mode.
        /// account_address -> new_default_deposit_mode
        deposit_mode_updates: HashMap<AccountAddress, DepositRule>,
        /// Additions to the authorized depositors
        authorized_depositors_added:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
        /// Removals from the authorized depositors
        authorized_depositors_removed:
            HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
    },

    /// A manifest that contributed some amount of resources to a liquidity
    /// pool that can be a one-resource pool, two-resource pool, or a
    /// multi-resource pool.
    PoolContribution {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,
        /// The contribution observed in the transaction
        pool_contributions: Vec<TrackedPoolContribution>,
    },

    /// A manifest that redeemed resources from a liquidity pool. Similar to
    /// contributions, this can be any of the three pool blueprints available
    /// in the pool package.
    PoolRedemption {
        /// The addresses of the pools in the transaction
        pool_addresses: Vec<PoolAddress>,

        /// The redemptions observed in the transaction
        pool_redemptions: Vec<TrackedPoolRedemption>,
    },

    /// A manifest that deletes accounts.
    DeleteAccounts {
        /// The addresses of the accounts that are being deleted
        account_addresses: Vec<AccountAddress>,
    },

    /// A manifest that is presented when a provisional security structure is applied
    /// to an entity
    SecurifyEntity {
        /// The entity address that is about to be securified
        entities: Vec<AddressOfAccountOrPersona>,
    },
    /// A manifest that creates accounts or personas, each with its owner
    /// badge in a new access controller, so they're securified from the
    /// start: e.g. a dApp setting up a shared account.
    SecurifiedEntityCreation {
        created_entities: Vec<CreatedSecurifiedEntity>,
    },
    /// A manifest that proposes new rules for access controllers and confirms
    /// them in the same transaction: the rules change once it's committed.
    AccessControllerRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        /// The recoveries proposed and confirmed in the manifest, with the
        /// rules they enact.
        recovery_proposals: Vec<AccessControllerRecoveryProposal>,
    },
    /// A manifest that starts a timed recovery of access controllers: the
    /// recovery role proposes rules without confirming them.
    AccessControllerTimedRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        timed_recoveries: Vec<AccessControllerTimedRecoveryProposal>,
    },
    /// A manifest that confirms recoveries already proposed on access
    /// controllers.
    AccessControllerConfirmRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        confirmations: Vec<AccessControllerRecoveryConfirmation>,
    },
    /// A manifest that stops or cancels recoveries already proposed on access
    /// controllers.
    AccessControllerCancelRecovery {
        ac_addresses: Vec<AccessControllerAddress>,
        cancellations: Vec<AccessControllerRecoveryCancellation>,
    },
    /// A manifest that locks or unlocks the primary role of access
    /// controllers.
    AccessControllerPrimaryRoleLock {
        ac_addresses: Vec<AccessControllerAddress>,
        operations: Vec<AccessControllerPrimaryRoleLockOperation>,
    },
    /// A manifest that tops up the recovery fee vaults of access controllers
    /// with XRD withdrawn from accounts, or withdraws XRD from them into
    /// accounts. Only one side is filled: `contributions` and
    /// `account_withdrawals` for a top-up, `withdrawals` and
    /// `account_deposits` for a withdrawal.
    AccessControllerRecoveryFee {
        ac_addresses: Vec<AccessControllerAddress>,
        contributions: Vec<AccessControllerXrdTransfer>,
        withdrawals: Vec<AccessControllerXrdTransfer>,
        account_withdrawals: Vec<AccountXrdTransfer>,
        account_deposits: Vec<AccountXrdTransfer>,
    },
}

delegate_display_debug_into!(
    DetailedManifestClass,
    InternalDetailedManifestClass
);
