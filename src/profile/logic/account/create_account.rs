use crate::prelude::*;
use std::future::Future;

impl Profile {
    /// Creates a new non securified account **WITHOUT** add it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `wallet.add_account(account)`
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
        let bdfs = self.bdfs();
        let index = self
            .next_derivation_index_for_entity(EntityKind::Account, network_id);

        let number_of_accounts_on_network = self
            .networks
            .get(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let appearance_id = AppearanceID::from_number_of_accounts_on_network(
            number_of_accounts_on_network,
        );

        let factor_instance =
            load_private_device_factor_source(bdfs).await.map(|p| {
                p.derive_entity_creation_factor_instance(network_id, index)
            })?;

        let account = Account::new(factor_instance, name, appearance_id);

        Ok(account)
    }
}
