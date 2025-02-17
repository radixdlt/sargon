use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsFactorSourceAdder {
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool>;

    async fn add_new_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsFactorSourceAdder for SargonOS {
    /// Accesses the active profile and checks if it already contains a factor source
    /// with the same `FactorSourceID`.
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool> {
        self.profile_contains_factor_source(factor_source_id).await
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
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<()> {
        let host_info = self.resolve_host_info().await;
        let factor_source = FactorSource::with_details(
            factor_source_kind,
            mnemonic_with_passphrase.clone(),
            name,
            host_info,
        )?;
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

        if factor_source_kind == FactorSourceKind::Device {
            self.secure_storage
                .save_mnemonic_with_passphrase(
                    &mnemonic_with_passphrase,
                    &factor_source.id_from_hash(),
                )
                .await?;
        }

        self.event_bus
            .emit(EventNotification::profile_modified(
                EventProfileModified::FactorSourceAdded { id },
            ))
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::prelude::CommonError::ProfileStateNotLoaded;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn is_factor_source_already_in_use() {
        let test = async |fsid: FactorSourceID,
                          expected_result: Result<bool>| {
            let mwp = MnemonicWithPassphrase::sample();
            let os = SUT::fast_boot_bdfs(mwp.clone()).await;

            let is_already_in_use =
                os.is_factor_source_already_in_use(fsid).await;

            pretty_assertions::assert_eq!(is_already_in_use, expected_result);
        };

        test(FactorSourceID::sample(), Ok(true)).await;
        test(FactorSourceID::sample_other(), Ok(false)).await;
    }

    #[actix_rt::test]
    async fn is_factor_source_already_in_use_error() {
        let bios = Bios::new(Drivers::test());
        let clients = Clients::new(bios);
        let interactors = Interactors::new_from_clients(&clients);
        let os =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let result = os
            .is_factor_source_already_in_use(FactorSourceID::sample())
            .await;

        assert!(matches!(result, Err(ProfileStateNotLoaded { .. })));
    }
}
