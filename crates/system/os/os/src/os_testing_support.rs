#![allow(non_snake_case)]

#[cfg(test)]
use crate::prelude::*;

#[cfg(test)]
impl SargonOS {
    pub async fn with_bdfs() -> (Arc<Self>, FactorSource) {
        let os = Self::fast_boot().await;
        let bdfs = os.bdfs().unwrap();
        (os, bdfs.into())
    }

    pub async fn create_and_save_new_mainnet_account_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(display_name).await
    }

    pub(crate) async fn create_and_save_new_mainnet_persona(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_derivation_outcome(name)
            .await
            .map(|(p, _)| p)
    }

    pub(crate) async fn create_and_save_new_mainnet_account(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_derivation_outcome(name)
            .await
            .map(|(a, _)| a)
    }

    pub(crate) async fn create_and_save_new_persona_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name, None).await
    }

    pub(crate) async fn create_and_save_new_account_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub(crate) async fn create_and_save_new_mainnet_persona_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(display_name).await
    }

    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    pub(crate) fn __OFFLINE_ONLY_securify_account_without_saving(
        &self,
        account_address: AccountAddress,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<Account> {
        let entity = self.__OFFLINE_ONLY_securify_entity_without_saving(
            AddressOfAccountOrPersona::Account(account_address),
            security_structure_of_factor_instances,
        )?;

        entity
            .clone()
            .as_account_entity()
            .ok_or(CommonError::ExpectedAccountButGotPersona {
                address: entity.address().to_string(),
            })
            .cloned()
    }

    pub(crate) fn __OFFLINE_ONLY_securify_entity_without_saving(
        &self,
        entity_address: AddressOfAccountOrPersona,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<AccountOrPersona> {
        let mut entity = self.entity_by_address(entity_address)?;

        let veci: HierarchicalDeterministicFactorInstance;
        let access_controller_address: AccessControllerAddress;

        match entity.security_state() {
            EntitySecurityState::Unsecured { value } => {
                veci = value.transaction_signing.clone();
                // THIS IS COMPLETELY WRONG!
                // The real solution should get the AccessControllerAddress on chain
                access_controller_address =
                    AccessControllerAddress::with_node_id_of(&entity.address());
            }
            EntitySecurityState::Securified { value } => {
                veci = value.veci.clone().unwrap();
                access_controller_address = value.access_controller_address;
            }
        };

        let securified_control = SecuredEntityControl::new(
            veci,
            access_controller_address,
            security_structure_of_factor_instances,
        )?;

        entity.set_security_state(EntitySecurityState::Securified {
            value: securified_control,
        })?;

        Ok(entity)
    }

    /// Uses FactorInstancesProvider to get factor instances for the `shield`.
    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    pub(crate) async fn __OFFLINE_ONLY_securify_accounts(
        &self,
        account_addresses: IndexSet<AccountAddress>,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(Accounts, FactorInstancesProviderOutcome)> {
        let (entities, outcome) = self
            .__OFFLINE_ONLY_securify_entities(
                account_addresses.into_iter().map(Into::into).collect(),
                shield,
            )
            .await?;

        let accounts = entities
            .into_iter()
            .map(|e| e.into_account_entity().unwrap())
            .collect();
        Ok((accounts, outcome))
    }

    pub(crate) async fn __OFFLINE_ONLY_securify_entities(
        &self,
        entity_addresses: IndexSet<AddressOfAccountOrPersona>,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(
        IdentifiedVecOf<AccountOrPersona>,
        FactorInstancesProviderOutcome,
    )> {
        entity_addresses
            .iter()
            .for_each(|a| assert!(self.entity_by_address(*a).is_ok()));

        let outcome = self.make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            entity_addresses.clone().into_iter().map(Into::into).collect(),
                    shield.clone()).await?;

        let (
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = outcome;

        let mut security_structures_of_factor_instances =
            security_structures_of_factor_instances;

        // consume!
        instances_in_cache_consumer.consume().await?;

        let securified_entities = entity_addresses
            .into_iter()
            .map(|entity_address| {
                let security_structure_of_factor_instances =
                    security_structures_of_factor_instances
                        .shift_remove(&entity_address)
                        .unwrap();

                // Production ready code should batch update entities, submit batch transaction to
                // network, and then batch update all accounts in Profile.
                self.__OFFLINE_ONLY_securify_entity_without_saving(
                    entity_address,
                    security_structure_of_factor_instances,
                )
            })
            .collect::<Result<IdentifiedVecOf<AccountOrPersona>>>()?;

        assert!(security_structures_of_factor_instances.is_empty());

        // Assert that none of the NEW FactorInstances collide with the existing ones
        self.profile()
            .unwrap()
            .assert_new_factor_instances_not_already_used_erased(
                securified_entities.clone(),
            )?;
        self.update_entities_erased(securified_entities.clone())
            .await?;

        Ok((
            securified_entities.into_iter().collect(),
            derivation_outcome,
        ))
    }

    /// Uses FactorInstancesProvider to get factor instances for the `shield`.
    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    #[allow(non_camel_case_types)]
    pub(crate) async fn __OFFLINE_ONLY_securify_account(
        &self,
        account_address: AccountAddress,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(Account, FactorInstancesProviderOutcome)> {
        let (accounts, outcome) = self
            .__OFFLINE_ONLY_securify_accounts(
                IndexSet::just(account_address),
                shield,
            )
            .await?;
        assert_eq!(accounts.len(), 1);
        let account = accounts.first().unwrap().clone();
        Ok((account, outcome))
    }
}
