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
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
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

    /// Uses FactorInstancesProvider to get factor instances for the `shield`.
    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    pub(crate) async fn __OFFLINE_ONLY_securify_accounts(
        &self,
        account_addresses: IndexSet<AccountAddress>,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(Accounts, FactorInstancesProviderOutcome)> {
        let (entities, outcome) = self
            ._apply_shield_to_entities_with_diagnostics(
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
