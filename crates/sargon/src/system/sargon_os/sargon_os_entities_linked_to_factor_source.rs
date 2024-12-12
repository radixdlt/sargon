use crate::prelude::*;

impl SargonOS {
    /// Returns the entities linked to a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToFactorSource> {
        let integrity = self.integrity(factor_source.clone()).await?;
        match profile_to_check {
            ProfileToCheck::Current => self
                .profile()?
                .current_network()?
                .entities_linked_to_factor_source(factor_source, integrity),
            ProfileToCheck::Specific(specific_profile) => {
                let profile_network = specific_profile
                    .networks
                    .get_id(NetworkID::Mainnet)
                    .ok_or(CommonError::Unknown)?;
                profile_network
                    .entities_linked_to_factor_source(factor_source, integrity)
            }
        }
    }

    async fn integrity(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorSourceIntegrity> {
        match factor_source {
            FactorSource::Device { value } => {
                self.device_integrity(value).await
            }
            FactorSource::Ledger { value } => Ok(value.into()),
            _ => Err(CommonError::Unknown),
        }
    }

    async fn device_integrity(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<FactorSourceIntegrity> {
        let is_mnemeonic_present_in_keychain = self
            .clients
            .secure_storage
            .contains_device_mnemonic(device_factor_source.clone())
            .await?;
        let is_mnemonic_marked_as_backed_up = self
            .clients
            .unsafe_storage
            .check_if_mnemonic_is_backed_up(device_factor_source.clone())
            .await?;
        let result = DeviceFactorSourceIntegrity::new(
            device_factor_source,
            is_mnemeonic_present_in_keychain,
            is_mnemonic_marked_as_backed_up,
        );
        Ok(result.into())
    }
}
