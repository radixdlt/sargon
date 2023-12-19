use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use wallet_kit_common::{error::common_error::CommonError, network_id::NetworkID};

use super::accounts::Accounts;

/// Accounts, Personas, Authorized dapps for some Radix Network that user
/// has created and interacted with.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    id: NetworkID,

    /// An ordered set of Accounts on this network.
    accounts: RefCell<Accounts>,
}

impl Network {
    /// Instantiates a new `Network` from `network_id` and `accounts`.
    ///
    /// Panics if not any account in `accounts` is on another
    /// network than `network_id`
    pub fn new(network_id: NetworkID, accounts: Accounts) -> Self {
        assert!(
            accounts
                .get_all()
                .into_iter()
                .all(|a| a.network_id() == network_id),
            "Discrepancy, found accounts on other network than {network_id}"
        );
        Self {
            id: network_id,
            accounts: RefCell::new(accounts),
        }
    }
}

impl Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub fn id(&self) -> NetworkID {
        self.id.clone()
    }

    /// An ordered set of Accounts on this network.
    pub fn accounts(&self) -> Accounts {
        self.accounts.borrow().clone()
    }
}

impl Network {
    /// Tries to change the accounts to `new`, will throw an error if any of the accounts in `new`
    /// is on a different network than `self.id()`.
    pub fn set_accounts(&self, new: Accounts) -> Result<(), CommonError> {
        if new.get_all().iter().any(|a| a.network_id() != self.id()) {
            return Err(CommonError::AccountOnWrongNetwork);
        }
        *self.accounts.borrow_mut() = new;
        Ok(())
    }
}

// CFG test
#[cfg(any(test, feature = "placeholder"))]
impl Network {
    pub fn placeholder() -> Self {
        Self::new(NetworkID::Mainnet, Accounts::placeholder())
    }

    pub fn placeholder_other() -> Self {
        Self::new(NetworkID::Mainnet, Accounts::placeholder_other())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::{error::common_error::CommonError, network_id::NetworkID};

    use crate::v100::{entity::account::account::Account, networks::network::accounts::Accounts};

    use super::Network;

    #[test]
    fn inequality() {
        assert_ne!(Network::placeholder(), Network::placeholder_other());
    }

    #[test]
    fn get_id() {
        assert_eq!(Network::placeholder().id(), NetworkID::Mainnet);
    }

    #[test]
    fn get_accounts() {
        let sut = Network::placeholder();
        assert_eq!(sut.accounts(), Accounts::placeholder());
    }

    #[test]
    fn set_accounts_wrong_network() {
        let sut = Network::placeholder();
        assert_eq!(
            sut.set_accounts(Accounts::with_account(Account::placeholder_stokenet())),
            Err(CommonError::AccountOnWrongNetwork)
        );
    }

    #[test]
    fn set_accounts_same_network() {
        assert_ne!(Accounts::placeholder(), Accounts::placeholder_other());
        let sut = Network::new(NetworkID::Mainnet, Accounts::placeholder());
        assert_eq!(sut.set_accounts(Accounts::placeholder_other()), Ok(()));
        assert_eq!(sut.accounts(), Accounts::placeholder_other());
    }

    #[test]
    fn duplicate_accounts_are_filtered_out() {
        assert_eq!(
            Network::new(
                NetworkID::Mainnet,
                Accounts::with_accounts(
                    [Account::placeholder(), Account::placeholder()].into_iter()
                )
            )
            .accounts()
            .len(),
            1
        )
    }

    #[test]
    #[should_panic(expected = "Discrepancy, found accounts on other network than mainnet")]
    fn panic_when_network_id_mismatch_between_accounts_and_value() {
        Network::new(
            NetworkID::Mainnet,
            Accounts::with_accounts(
                [
                    Account::placeholder_mainnet(),
                    Account::placeholder_stokenet(),
                ]
                .into_iter(),
            ),
        );
    }
}
