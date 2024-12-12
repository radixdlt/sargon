use crate::prelude::*;

impl SargonOS {
    /// Returns the entities linked to a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToFactorSource> {
        let accessibility = self.accessibility(factor_source.clone()).await?;
        match profile_to_check {
            ProfileToCheck::Current => {
                let accounts = self.accounts_on_current_network()?;
                let hidden_accounts = self
                    .profile_state_holder
                    .hidden_accounts_on_current_network()?;
                let personas = self.personas_on_current_network()?;
                let hidden_personas = self
                    .profile_state_holder
                    .hidden_personas_on_current_network()?;

                Ok(self.create_entities_linked_to_factor_source(
                    factor_source,
                    accessibility,
                    accounts,
                    hidden_accounts,
                    personas,
                    hidden_personas,
                ))
            }
            ProfileToCheck::Specific(specific_profile) => {
                let network = specific_profile
                    .networks
                    .get_id(NetworkID::Mainnet)
                    .ok_or(CommonError::Unknown)?;
                let accounts = network.accounts.visible();
                let hidden_accounts = network.accounts.hidden();
                let personas = network.personas.non_hidden();
                let hidden_personas = network.personas.hidden();

                Ok(self.create_entities_linked_to_factor_source(
                    factor_source,
                    accessibility,
                    accounts,
                    hidden_accounts,
                    personas,
                    hidden_personas,
                ))
            }
        }
    }

    fn create_entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        accessibility: FactorSourceAccessibility,
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> EntitiesLinkedToFactorSource {
        let accounts = accounts
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect();
        let hidden_accounts = hidden_accounts
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect();
        let personas = personas
            .iter()
            .filter(|p| {
                p.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect();
        let hidden_personas = hidden_personas
            .iter()
            .filter(|p| {
                p.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect();
        EntitiesLinkedToFactorSource {
            accessibility,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }

    async fn accessibility(
        &self,
        factor_source: FactorSource,
    ) -> Result<FactorSourceAccessibility> {
        match factor_source {
            FactorSource::Device { value } => {
                self.device_accessibility(value).await
            }
            FactorSource::Ledger { value } => Ok(value.into()),
            _ => Err(CommonError::Unknown),
        }
    }

    async fn device_accessibility(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<FactorSourceAccessibility> {
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
        let result = DeviceFactorSourceAccessibility::new(
            device_factor_source,
            is_mnemeonic_present_in_keychain,
            is_mnemonic_marked_as_backed_up,
        );
        Ok(result.into())
    }
}
