use serde::{Deserialize, Serialize};
use wallet_kit_common::network_id::NetworkID;

use super::accounts::Accounts;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Network {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub id: NetworkID,

    /// A non empty ordered set of Accounts on this network.
    pub accounts: Accounts,
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
                .all(|a| a.network_id == network_id),
            "Discrepancy, found accounts on other network than {network_id}"
        );
        Self {
            id: network_id,
            accounts,
        }
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::network_id::NetworkID;

    use crate::v100::{entity::account::account::Account, networks::network::accounts::Accounts};

    use super::Network;

    #[test]
    fn duplicate_accounts_are_filtered_out() {
        assert_eq!(
            Network::new(
                NetworkID::Mainnet,
                Accounts::with_accounts(
                    [Account::placeholder(), Account::placeholder()].into_iter()
                )
            )
            .accounts
            .len(),
            1
        )
    }

    #[test]
    #[should_panic(expected = "Discrepancy, found accounts on other network than mainnet\n")]
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
