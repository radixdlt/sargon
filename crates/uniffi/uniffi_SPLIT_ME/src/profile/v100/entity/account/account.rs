use crate::prelude::*;
use profile_logic::{
    AccountIsLegacy, EntityUnsecuredControllingFactorInstance,
};
use sargon::Account as InternalAccount;

decl_vec_samples_for!(Accounts, Account);

/// A network unique account with a unique public address and a set of cryptographic
/// factors used to control it.
///
/// Used to own and control assets on the radix network. Uniquely identified by an
/// account address, e.g.
///
/// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
///
/// But most commonly users see the address on its abbreviated form:
///
/// `acco...nvjdwr`
///
/// Accounts have a display name and an appearance id.
///
/// An account can be either controlled by a "Babylon" DeviceFactorSource or a
/// Legacy one imported from Olympia, or a Ledger hardware wallet, which too might
/// have been imported from Olympia.
#[derive(Clone, PartialEq, Hash, Eq, uniffi::Record)]
pub struct Account {
    /// The ID of the network this account can be used with.
    pub network_id: NetworkID,

    /// A globally unique identifier of this account, being a human readable
    /// address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...nvjdwr`
    ///
    /// No two addresses will ever be the same even for the same factor source
    /// but on different networks, since the public keys controlling the
    /// accounts depend on the network id.
    pub address: AccountAddress,

    /// An off-ledger display name or description chosen by the user when she
    /// created this account.
    pub display_name: DisplayName,

    /// Security state of this account, either "securified" or not.
    pub security_state: EntitySecurityState,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    pub appearance_id: AppearanceID,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    pub flags: Vec<EntityFlag>,

    /// The on ledger synced settings for this account, contains e.g.
    /// ThirdPartyDeposit settings, with deposit rules for assets.
    pub on_ledger_settings: OnLedgerSettings,
}

impl Account {
    pub fn into_internal(&self) -> InternalAccount {
        self.clone().into()
    }
}

impl From<Account> for InternalAccount {
    fn from(value: Account) -> Self {
        Self::with(
            value.network_id,
            value.address,
            value.display_name,
            value.security_state,
            value.flags.into_iter().map(Into::into),
            value.appearance_id,
            value.on_ledger_settings,
        )
    }
}

impl From<InternalAccount> for Account {
    fn from(value: InternalAccount) -> Self {
        Self {
            network_id: value.network_id.into(),
            address: value.address.into(),
            display_name: value.display_name.into(),
            security_state: value.security_state.clone().into(),
            appearance_id: value.appearance_id.into(),
            flags: value.flags.clone().into_iter().map(Into::into).collect(),
            on_ledger_settings: value.on_ledger_settings.into(),
        }
    }
}

#[uniffi::export]
pub fn new_account_sample_mainnet_alice() -> Account {
    InternalAccount::sample_mainnet_alice().into()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_bob() -> Account {
    InternalAccount::sample_mainnet_bob().into()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_carol() -> Account {
    InternalAccount::sample_mainnet_carol().into()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_diana() -> Account {
    InternalAccount::sample_mainnet_diana().into()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_nadia() -> Account {
    InternalAccount::sample_stokenet_nadia().into()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_olivia() -> Account {
    InternalAccount::sample_stokenet_olivia().into()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_paige() -> Account {
    InternalAccount::sample_stokenet_paige().into()
}

#[uniffi::export]
pub fn account_is_legacy(account: Account) -> bool {
    account.into_internal().is_legacy()
}

#[uniffi::export]
pub fn account_unsecured_controlling_factor_instance(
    account: Account,
) -> Option<HierarchicalDeterministicFactorInstance> {
    account
        .into_internal()
        .unsecured_controlling_factor_instance()
        .map(|key| key.into())
}
