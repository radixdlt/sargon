use crate::prelude::*;

#[async_trait::async_trait]
pub trait EntityCreatingWithFactorSourceAndDerivationOutcome {
    async fn create_unsaved_entity_with_factor_source<
        E: IsEntity + Identifiable,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<E>;

    async fn create_unsaved_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<Account> {
        self.create_unsaved_entity_with_factor_source(
            factor_source,
            network_id,
            name,
            key_derivation_interactor,
        )
        .await
    }

    async fn create_unsaved_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<Persona> {
        self.create_unsaved_entity_with_factor_source(
            factor_source,
            network_id,
            name,
            key_derivation_interactor,
        )
        .await
    }

    async fn create_unsaved_entities_with_factor_source_with_derivation_outcome<
        E: IsEntity + Identifiable,
        F: Send + Fn(u32) -> DisplayName,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of entity at index
    ) -> Result<(
        FactorSourceID,
        IdentifiedVecOf<E>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )>;

    async fn create_unsaved_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        FactorSourceID,
        Account,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let (
            factor_source_id,
            accounts,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = self
            .create_unsaved_accounts_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                1,
                factor_instances_cache_client,
                key_derivation_interactor,
                |_| name,
            )
            .await?;

        let account = accounts
            .into_iter()
            .last()
            .expect("Should have created one account");

        Ok((
            factor_source_id,
            account,
            instances_in_cache_consumer,
            derivation_outcome,
        ))
    }

    async fn create_unsaved_accounts_with_factor_source_with_derivation_outcome<
        F,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of account at index
    ) -> Result<(
        FactorSourceID,
        Accounts,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )>
    where
        F: Send + Fn(u32) -> DisplayName;

    async fn create_unsaved_accounts_with_factor_source<F>(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of account at index
    ) -> Result<(FactorSourceID, Accounts, InstancesInCacheConsumer)>
    where
        F: Send + Fn(u32) -> DisplayName,
    {
        self.create_unsaved_accounts_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactor,
            get_name,
        )
        .await
        .map(|(x, y, z, _)| (x, y, z))
    }

    async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        FactorSourceID,
        Persona,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let (
            factor_source_id,
            personas,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = self
            .create_unsaved_personas_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                1,
                factor_instances_cache_client,
                key_derivation_interactor,
                |_| name,
            )
            .await?;

        let persona = personas
            .into_iter()
            .last()
            .expect("Should have created one persona");

        Ok((
            factor_source_id,
            persona,
            instances_in_cache_consumer,
            derivation_outcome,
        ))
    }

    async fn create_unsaved_personas_with_factor_source_with_derivation_outcome<
        F,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of persona at index
    ) -> Result<(
        FactorSourceID,
        Personas,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )>
    where
        F: Send + Fn(u32) -> DisplayName,
    {
        self.create_unsaved_entities_with_factor_source_with_derivation_outcome::<Persona, _>(
            factor_source,
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactor,
            get_name,
        )
        .await.map(|(a, b, c, d)| (a, b.into_iter().collect(), c, d))
    }
}

#[async_trait::async_trait]
impl EntityCreatingWithFactorSourceAndDerivationOutcome for Profile {
    async fn create_unsaved_accounts_with_factor_source_with_derivation_outcome<
        F,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of account at index
    ) -> Result<(
        FactorSourceID,
        Accounts,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )>
    where
        F: Send + Fn(u32) -> DisplayName,
    {
        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let (factor_source_id, accounts, instances_in_cache_consumer, derivation_outcome) = self
            .create_unsaved_entities_with_factor_source_with_derivation_outcome::<Account, _>(
                factor_source,
                network_id,
                count,
                factor_instances_cache_client,
                key_derivation_interactor,
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
            instances_in_cache_consumer,
            derivation_outcome,
        ))
    }

    async fn create_unsaved_entity_with_factor_source<
        E: IsEntity + Identifiable,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<E> {
        let next_derivation_index_assigner =
            NextDerivationEntityIndexProfileAnalyzingAssigner::new(
                network_id,
                Some(Arc::new(self.clone())),
            );
        let index_agnostic_path = IndexAgnosticPath::new(
            network_id,
            E::entity_kind(),
            CAP26KeyKind::TransactionSigning,
            KeySpace::Unsecurified { is_hardened: true },
        );
        let default_index = HDPathComponent::from_local_key_space(
            0u32,
            index_agnostic_path.key_space,
        )?;
        let next_derivation_index = next_derivation_index_assigner
            .next(factor_source.id_from_hash(), index_agnostic_path)?
            .unwrap_or(default_index);

        let derivation_path =
            DerivationPath::from_index_agnostic_path_and_component(
                index_agnostic_path,
                next_derivation_index,
            );
        let purpose = match E::entity_kind() {
            CAP26EntityKind::Account => DerivationPurpose::CreatingNewAccount,
            CAP26EntityKind::Identity => DerivationPurpose::CreatingNewPersona,
        };

        let derive_request = KeyDerivationRequest::new_mono_factor(
            purpose,
            factor_source.id_from_hash(),
            IndexSet::from([derivation_path]),
        );

        let key_derivation_response =
            key_derivation_interactor.derive(derive_request).await?;

        let derived_instance = key_derivation_response
            .per_factor_source
            .get(&factor_source.id_from_hash())
            .expect("Response should contain the requested factor source")
            .first()
            .expect("Response should contain the derive instance");

        let entity = E::with_veci_and_name(
            HDFactorInstanceTransactionSigning::<E::Path>::new(
                derived_instance.clone(),
            )?,
            name,
        );
        Ok(entity)
    }

    /// Creates `count` many new virtual entities of type `E` on `network_id` with `factor_source` as the factor source.
    /// Setting the names according to `get_name`, loading pre-derived FactorInstances from the
    /// FactorInstancesCache if possible, else derives more using the FactorInstancesProvider.
    ///
    /// Returns the FactorSourceID, the entities, the InstancesInCacheConsumer, and the FactorInstancesProviderOutcomeForFactor.
    ///
    /// The `FactorInstancesProviderOutcomeForFactor` is primarily useful for testing.
    ///
    /// The `InstancesInCacheConsumer` SHOULD be called by the caller, once you know it
    /// is safe to delete the instances from the cache - e.g. after having saved the new
    /// entities into the Profile and persisted it into SecureStorage.
    async fn create_unsaved_entities_with_factor_source_with_derivation_outcome<
        E,
        F,
    >(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: F, // name of entity at index
    ) -> Result<(
        FactorSourceID,
        IdentifiedVecOf<E>,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )>
    where
        E: IsEntity + Identifiable,
        F: Send + Fn(u32) -> DisplayName,
    {
        let count = count as usize;

        let fsid = factor_source.factor_source_id();
        let entity_kind = E::entity_kind();

        let (instances_in_cache_consumer, outcome) =
            VirtualEntityCreatingInstanceProvider::for_many_entity_vecis(
                count,
                entity_kind,
                factor_instances_cache_client,
                Arc::new(self.clone()),
                factor_source.clone(),
                network_id,
                key_derivation_interactor,
            )
            .await?;

        let outcome = outcome
            .per_derivation_preset
            .get(&DerivationPreset::veci_entity_kind(entity_kind))
            .unwrap()
            .per_factor
            .get(&factor_source.id_from_hash())
            .cloned()
            .unwrap();

        let instances_to_use_directly = outcome.clone().to_use_directly;

        assert_eq!(instances_to_use_directly.len(), count);

        let entities = instances_to_use_directly
            .into_iter()
            .map(|f| {
                HDFactorInstanceTransactionSigning::<E::Path>::new(f).unwrap()
            })
            .map(|veci| {
                let idx = u32::from(
                    veci.path
                        .derivation_path()
                        .index()
                        .index_in_local_key_space(),
                );
                let name = get_name(idx);

                E::with_veci_and_name(veci, name)
            })
            .collect::<IdentifiedVecOf<E>>();

        Ok((fsid, entities, instances_in_cache_consumer, outcome))
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
        let secure_storage_client = Arc::new(secure_storage_client);
        let interactors = Arc::new(TestDerivationInteractor::new(
            false,
            secure_storage_client.clone(),
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
