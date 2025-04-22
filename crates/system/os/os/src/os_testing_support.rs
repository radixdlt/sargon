#![allow(non_snake_case)]

use crate::prelude::*;

#[cfg(debug_assertions)]
impl SargonOS {
    pub async fn with_bdfs() -> (Arc<Self>, FactorSource) {
        let os = Self::fast_boot().await;
        let bdfs = os.main_bdfs().unwrap();
        (os, bdfs.into())
    }

    pub async fn create_and_save_new_mainnet_account_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_account_with_main_bdfs_with_derivation_outcome(display_name).await
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
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name, None).await
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
        self.create_and_save_new_mainnet_persona_with_main_bdfs_with_derivation_outcome(display_name).await
    }


    pub async fn create_and_save_new_mainnet_account_with_main_bdfs_with_derivation_outcome(
        &self,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.main_bdfs()?;
        self.create_and_save_new_mainnet_account_with_factor_source_with_derivation_outcome(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Create a new mainnet Account using the selected factor source and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::AccountAdded }`
    pub async fn create_and_save_new_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_and_save_new_account_with_factor_source(factor_source, NetworkID::Mainnet, name).await
    }

    pub async fn create_and_save_new_mainnet_account_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }

         /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the main BDFS.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_main_bdfs(
        &self,
    ) -> Result<Account> {
        let bdfs = self.main_bdfs()?;
        self.create_unsaved_unnamed_mainnet_account_with_factor_source(
            bdfs.into(),
        )
        .await
    }

    /// Creates a new unsaved mainnet account named "Unnamed {N}", where `N` is the
    /// index of the next account for the selected factor_source.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`
    pub async fn create_unsaved_unnamed_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<Account> {
        self.create_unsaved_account_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            DisplayName::new("Unnamed").unwrap(),
        )
        .await
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using main BDFS.
    pub async fn create_unsaved_mainnet_account_with_main_bdfs(
        &self,
        name: DisplayName,
    ) -> Result<Account> {
        let bdfs = self.main_bdfs()?;
        self.create_unsaved_mainnet_account_with_factor_source(
            bdfs.into(),
            name,
        )
        .await
    }

    /// Uses `create_unsaved_account` specifying `NetworkID::Mainnet` using
    /// the specified `factor_source`.
    pub async fn create_unsaved_mainnet_account_with_factor_source(
        &self,
        factor_source: FactorSource,
        name: DisplayName,
    ) -> Result<Account> {
        self.create_unsaved_account_with_factor_source(
            factor_source,
            NetworkID::Mainnet,
            name,
        )
        .await
    }
}
