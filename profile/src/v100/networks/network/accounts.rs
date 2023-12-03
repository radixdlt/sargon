use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::v100::{address::account_address::AccountAddress, entity::account::account::Account};

/// An ordered set of Accounts on a specific network, most commonly
/// the set is non-empty.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct Accounts(BTreeMap<AccountAddress, Account>);

impl Accounts {
    /// Instantiates a new collection of accounts from an ordered
    /// map of accounts.
    pub fn new(accounts: BTreeMap<AccountAddress, Account>) -> Self {
        Self(accounts)
    }

    /// Instantiates a new collection of accounts from
    /// and iterator of accounts.
    pub fn with_accounts<I>(accounts: I) -> Self
    where
        I: Iterator<Item = Account>,
    {
        Self::new(accounts.map(|a| (a.address.clone(), a)).collect())
    }

    /// Instantiates a new collection of accounts from a
    /// single account.
    pub fn with_account(account: Account) -> Self {
        Self::with_accounts([account].into_iter())
    }
}

// Getters
impl Accounts {
    /// Returns the number of accounts.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a reference to the account identified by `address`, if it exists.
    pub fn get_account_by_address(&self, address: &AccountAddress) -> Option<&Account> {
        self.0.get(address)
    }

    /// Returns references to **all** accounts, including hidden ones.
    pub fn get_all(&self) -> Vec<&Account> {
        self.0.iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::v100::{
        address::account_address::AccountAddress,
        entity::{
            account::{account::Account, appearance_id::AppearanceID},
            display_name::DisplayName,
        },
        networks::network::accounts::Accounts,
    };

    #[test]
    fn default_is_empty() {
        assert_eq!(Accounts::default().len(), 0);
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            Accounts::with_accounts([Account::placeholder(), Account::placeholder()].into_iter())
                .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(Accounts::with_account(Account::placeholder()).len(), 1)
    }

    #[test]
    fn get_by_address() {
        let address: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();
        let account = Account::placeholder_with_values(
            address.clone(),
            DisplayName::default(),
            AppearanceID::default(),
        );
        let accounts = Accounts::with_account(account.clone());
        assert_eq!(accounts.get_account_by_address(&address), Some(&account));
    }
}
