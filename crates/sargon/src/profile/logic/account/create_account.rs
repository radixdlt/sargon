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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Account)> {
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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Account)> {
        let (factor_source_id, accounts) = self
            .create_unsaved_accounts_with_factor_source(
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

        Ok((factor_source_id, account))
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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(FactorSourceID, Accounts)> {
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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(FactorSourceID, Accounts)> {
        let count = count as usize;

        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let fsid = factor_source.factor_source_id();

        let outcome =
            VirtualEntityCreatingInstanceProvider::for_many_account_vecis(
                count,
                factor_instances_cache_client,
                Some(self),
                factor_source,
                network_id,
                key_derivation_interactors,
            )
            .await?;

        let instances_to_use_directly = outcome.to_use_directly;

        assert_eq!(instances_to_use_directly.len(), count);

        let accounts = instances_to_use_directly
            .into_iter()
            .map(|f| {
                HDFactorInstanceTransactionSigning::<AccountPath>::new(f)
                    .unwrap()
            })
            .map(|veci| {
                let idx =
                    u32::from(veci.path.index().index_in_local_key_space());
                let name = get_name(idx);
                let appearance_id =
                    AppearanceID::from_number_of_accounts_on_network(
                        (idx as usize) + number_of_accounts_on_network,
                    );

                Account::new(veci, name, appearance_id)
            })
            .collect::<Accounts>();

        Ok((fsid, accounts))
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

        let cache_client = FactorInstancesCacheClient::in_memory();

        let interactors = Arc::new(
            TestDerivationInteractors::mono_and_poly_with_extra_mnemonics(
                IndexMap::kv(
                    fs.factor_source.id_from_hash(),
                    fs.mnemonic_with_passphrase.clone(),
                ),
            ),
        );

        let (_, accounts) = sut
            .create_unsaved_accounts_with_factor_source(
                fs.factor_source.clone().into(),
                NetworkID::Mainnet,
                3,
                &cache_client,
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
