#![allow(non_snake_case)]

use crate::prelude::*;

#[cfg(debug_assertions)]
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

    pub async fn create_and_save_new_mainnet_persona(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_derivation_outcome(name)
            .await
            .map(|(p, _)| p)
    }

    pub async fn create_and_save_new_mainnet_account(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_derivation_outcome(name)
            .await
            .map(|(a, _)| a)
    }

    pub async fn create_and_save_new_persona_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub async fn create_and_save_new_account_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub async fn create_and_save_new_mainnet_persona_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(display_name).await
    }
}
