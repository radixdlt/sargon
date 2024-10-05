use crate::prelude::*;
use sargon::Account as InternalAccount;

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
#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{display_name} | {address}")]
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
    pub flags: EntityFlags,

    /// The on ledger synced settings for this account, contains e.g.
    /// ThirdPartyDeposit settings, with deposit rules for assets.
    pub on_ledger_settings: OnLedgerSettings,
}

impl From<InternalAccount> for Account {
    fn from(value: InternalAccount) -> Self {
        unimplemented!()
    }
}

impl Into<InternalAccount> for Account {
    fn into(self) -> InternalAccount {
        unimplemented!()
    }
}

impl Identifiable for Account {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

#[uniffi::export]
pub fn new_account_sample_mainnet_alice() -> Account {
    Account::sample_mainnet_alice()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_bob() -> Account {
    Account::sample_mainnet_bob()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_carol() -> Account {
    Account::sample_mainnet_carol()
}

#[uniffi::export]
pub fn new_account_sample_mainnet_diana() -> Account {
    Account::sample_mainnet_diana()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_nadia() -> Account {
    Account::sample_stokenet_nadia()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_olivia() -> Account {
    Account::sample_stokenet_olivia()
}

#[uniffi::export]
pub fn new_account_sample_stokenet_paige() -> Account {
    Account::sample_stokenet_paige()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Account;

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_sample_mainnet_alice(),
                new_account_sample_mainnet_bob(),
                new_account_sample_mainnet_carol(),
                new_account_sample_mainnet_diana(),
                new_account_sample_stokenet_nadia(),
                new_account_sample_stokenet_olivia(),
                new_account_sample_stokenet_paige(),
                // duplicates should be removed
                new_account_sample_mainnet_alice(),
                new_account_sample_mainnet_bob(),
                new_account_sample_mainnet_carol(),
                new_account_sample_mainnet_diana(),
                new_account_sample_stokenet_nadia(),
                new_account_sample_stokenet_olivia(),
                new_account_sample_stokenet_paige(),
            ])
            .len(),
            7
        )
    }
}
