use crate::prelude::*;
use std::future::Future;

impl Profile {
    pub fn current_gateway(&self) -> Gateway {
        self.app_preferences.gateways.current.clone()
    }

    pub fn current_network_id(&self) -> NetworkID {
        self.current_gateway().network.id
    }

    pub fn current_network(&self) -> &ProfileNetwork {
        self.networks
            .get_id(self.current_network_id())
            .expect("Should have current network")
    }

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

    /// Creates a new non securified account **WITHOUT** add it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_account(account)`
    pub async fn create_unsaved_account<F, Fut>(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        load_private_device_factor_source: F,
    ) -> Result<Account>
    where
        F: FnOnce(DeviceFactorSource) -> Fut,
        Fut: Future<
            Output = Result<PrivateHierarchicalDeterministicFactorSource>,
        >,
    {
        let accounts = self
            .create_unsaved_accounts(
                network_id,
                1,
                |_| name.clone(),
                load_private_device_factor_source,
            )
            .await?;

        let account = accounts
            .into_iter()
            .last()
            .expect("Should have created one account");

        Ok(account)
    }

    /// Creates many new non securified accounts **WITHOUT** add them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation paths.
    ///
    /// If you want to add the accounts to Profile, call `add_accounts(accounts)`
    pub async fn create_unsaved_accounts<F, Fut>(
        &self,
        network_id: NetworkID,
        count: u16,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
        load_private_device_factor_source: F,
    ) -> Result<Accounts>
    where
        F: FnOnce(DeviceFactorSource) -> Fut,
        Fut: Future<
            Output = Result<PrivateHierarchicalDeterministicFactorSource>,
        >,
    {
        let index = self
            .next_derivation_index_for_entity(EntityKind::Account, network_id);

        assert!((index as i64) - (count as i64) < (u32::MAX as i64)); // unlikely edge case

        let bdfs = self.bdfs();
        let count = count as u32;

        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let indices = index..index + count;

        let factor_instances = load_private_device_factor_source(bdfs.clone())
            .await
            .map(|p| {
                assert_eq!(p.factor_source, bdfs);
                p.derive_entity_creation_factor_instances(network_id, indices)
            })?;

        let accounts = factor_instances
            .into_iter()
            .map(|f| {
                let idx = f.index();
                let name = get_name(idx);
                let appearance_id =
                    AppearanceID::from_number_of_accounts_on_network(
                        (idx as usize) + number_of_accounts_on_network,
                    );

                Account::new(f, name, appearance_id)
            })
            .collect::<Accounts>();

        Ok(accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_unsaved_accounts() {
        let sut =
            Profile::new(DeviceFactorSource::sample(), DeviceInfo::sample());

        let accounts = sut
            .create_unsaved_accounts(
                NetworkID::Mainnet,
                3,
                |i| {
                    DisplayName::new(if i == 0 {
                        "Alice"
                    } else if i == 1 {
                        "Bob"
                    } else {
                        "Carol"
                    })
                    .unwrap()
                },
                async move |_| {
                    Ok(PrivateHierarchicalDeterministicFactorSource::sample())
                },
            )
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            accounts,
            Accounts::from_iter([
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob(),
                Account::sample_mainnet_carol()
            ])
        )
    }
}
