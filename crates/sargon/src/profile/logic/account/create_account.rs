use crate::prelude::*;
use std::{future::Future, pin::Pin};

impl Profile {
    /// Creates a new non securified account **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_account(account)`
    ///
    /// Returns a tuple `(FactorSourceID, Account)` where FactorSourceID is the ID
    /// of the FactorSource used to create the account.
    pub async fn create_unsaved_account_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Account, InstancesConsumer)> {
        let bdfs = self.bdfs();
        self.create_unsaved_account_with_factor_source(
            bdfs.into(),
            network_id,
            name,
            factor_instances_cache_client,
            key_derivation_interactors,
        )
        .await
    }

    /// Creates a new non securified account **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_account(account)`
    ///
    /// Returns a tuple `(FactorSourceID, Account)` where FactorSourceID is the ID
    /// of the FactorSource used to create the account.
    pub async fn create_unsaved_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Account, InstancesConsumer)> {
        self.create_unsaved_account_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
            factor_instances_cache_client,
            key_derivation_interactors,
        )
        .await
        .map(|(x, y, z, _)| (x, y, z))
    }

    pub async fn create_unsaved_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(
        FactorSourceID,
        Account,
        InstancesConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let (
            factor_source_id,
            accounts,
            instances_consumer,
            derivation_outcome,
        ) = self
            .create_unsaved_accounts_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                1,
                factor_instances_cache_client,
                key_derivation_interactors,
                |_| name.clone(),
            )
            .await?;

        let account = accounts
            .into_iter()
            .last()
            .expect("Should have created one account");

        Ok((
            factor_source_id,
            account,
            instances_consumer,
            derivation_outcome,
        ))
    }

    /// Creates many new non securified accounts **WITHOUT** adding them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add the accounts to Profile, call `add_accounts(accounts)`
    ///
    /// Returns a tuple `(FactorSourceID, Accounts)` where FactorSourceID is the ID
    /// of the FactorSource used to create the accounts.
    pub async fn create_unsaved_accounts_with_bdfs(
        &self,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(FactorSourceID, Accounts, InstancesConsumer)> {
        let bdfs = self.bdfs();
        self.create_unsaved_accounts_with_factor_source(
            bdfs.into(),
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactors,
            get_name,
        )
        .await
    }

    pub async fn create_unsaved_accounts_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(FactorSourceID, Accounts, InstancesConsumer)> {
        self.create_unsaved_accounts_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactors,
            get_name,
        )
        .await
        .map(|(x, y, z, _)| (x, y, z))
    }

    pub async fn create_unsaved_accounts_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(
        FactorSourceID,
        Accounts,
        InstancesConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let (factor_source_id, accounts, instances_consumer, derivation_outcome) = self
            .create_unsaved_entities_with_factor_source_with_derivation_outcome::<Account>(
                factor_source,
                network_id,
                count,
                factor_instances_cache_client,
                key_derivation_interactors,
                get_name,
            )
            .await?;

        let accounts_with_appearance_ids_set = accounts
            .into_iter()
            .enumerate()
            .map(|(offset, account)| {
                let mut account = account;
                let appearance_id =
                    AppearanceID::from_number_of_accounts_on_network(
                        number_of_accounts_on_network + offset,
                    );
                account.appearance_id = appearance_id;
                account
            })
            .collect::<Accounts>();

        Ok((
            factor_source_id,
            accounts_with_appearance_ids_set,
            instances_consumer,
            derivation_outcome,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_unsaved_accounts() {
        let fs = PrivateHierarchicalDeterministicFactorSource::sample();
        let sut = Profile::from_device_factor_source(
            fs.factor_source.clone(),
            HostId::sample(),
            HostInfo::sample(),
            None::<Accounts>,
        );

        let cache_client = Arc::new(FactorInstancesCacheClient::in_memory());
        let (secure_storage_client, _) = SecureStorageClient::ephemeral();
        secure_storage_client
            .save_private_hd_factor_source(&fs)
            .await
            .unwrap();
        let interactors =
            Arc::new(TestDerivationInteractors::with_secure_storage(
                secure_storage_client,
            ));

        let (_, accounts, consumer) = sut
            .create_unsaved_accounts_with_factor_source(
                fs.factor_source.clone().into(),
                NetworkID::Mainnet,
                3,
                cache_client,
                interactors,
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
            )
            .await
            .unwrap();
        consumer.consume().await.unwrap();

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
