use crate::prelude::*;
use std::future::Future;

impl Profile {
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
            .get_id(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let indices = index..index + count;

        let factor_instances =
            load_private_device_factor_source(bdfs).await.map(|p| {
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