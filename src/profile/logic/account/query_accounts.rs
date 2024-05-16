use crate::prelude::*;

impl Profile {
    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Accounts {
        self.current_network().accounts.non_hidden()
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> AccountsForDisplay {
        self.accounts_on_current_network()
            .iter()
            .map(AccountForDisplay::from)
            .collect::<AccountsForDisplay>()
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        for network in self.networks.iter() {
            if let Some(account) = network.accounts.get_id(address) {
                return Ok(account.clone());
            }
        }
        Err(CommonError::UnknownAccount)
    }
}
