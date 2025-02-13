use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsFactorSourceAdder: Send + Sync {
    async fn is_factor_source_already_in_use(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool>;

    async fn add_new_factor_source(
        &self,
        factor_source: FactorSource,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Result<()>;

    async fn resolve_host_info(&self) -> HostInfo;
}

#[async_trait::async_trait]
impl OsFactorSourceAdder for Arc<SargonOS> {
    /// Accesses the active profile and checks if it already contains a factor source
    /// with the same `FactorSourceID`.
    async fn is_factor_source_already_in_use(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        self.profile_contains_factor_source(factor_source.factor_source_id())
            .await
    }

    /// Returns `Err(CommonError::FactorSourceAlreadyExists)` if the Profile already contained a
    /// factor source with the same id.
    ///
    /// Pre-derives and fill cache with instances for the `factor_source`. If failed to pre-derive,
    /// removes the factor source from the profile and returns the error. If successful,
    /// saves the mnemonic to secure storage.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceAdded }`
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    async fn add_new_factor_source(
        &self,
        factor_source: FactorSource,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Result<()> {
        let id = factor_source.factor_source_id();

        let contains = self.profile_contains_factor_source(id).await?;

        if contains {
            return Err(CommonError::FactorSourceAlreadyExists);
        }

        self.update_profile_with(|p| {
            p.factor_sources.append(factor_source.clone());
            Ok(())
        })
        .await?;

        if let Err(e) = self
            .pre_derive_and_fill_cache_with_instances_for_factor_source(
                factor_source.clone(),
            )
            .await
        {
            self.update_profile_with(|p| {
                p.factor_sources.remove_id(&id);
                Ok(())
            })
            .await?;

            return Err(e);
        }

        self.secure_storage
            .save_mnemonic_with_passphrase(
                &mnemonic_with_passphrase,
                &factor_source.id_from_hash(),
            )
            .await?;

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceAdded { id },
            ))
            .await;

        Ok(())
    }

    /// Resolves the host info. This is used to create a new `DeviceFactorSource`.
    async fn resolve_host_info(&self) -> HostInfo {
        self.host.resolve_host_info().await
    }
}
