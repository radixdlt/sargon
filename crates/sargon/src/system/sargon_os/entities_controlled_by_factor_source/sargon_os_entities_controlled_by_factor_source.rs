use crate::prelude::*;

impl SargonOS {
    /// Returns the entities controlled by a given `FactorSource`, either on the current `Profile` or a specific one.
    pub async fn entities_controlled_by_factor_source(
        &self,
        factor_source: FactorSource,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesControlledByFactorSource> {
        let is_mnemonic_present_in_keychain = self
            .is_mnemonic_present_in_keychain(factor_source.clone())
            .await?;
        let is_mnemonic_marked_as_backed_up = self
            .is_mnemonic_marked_as_backed_up(factor_source.clone())
            .await?;
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

                Ok(self.create_entities_controlled_by_factor_source(
                    factor_source,
                    is_mnemonic_present_in_keychain,
                    is_mnemonic_marked_as_backed_up,
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

                Ok(self.create_entities_controlled_by_factor_source(
                    factor_source,
                    is_mnemonic_present_in_keychain,
                    is_mnemonic_marked_as_backed_up,
                    accounts,
                    hidden_accounts,
                    personas,
                    hidden_personas,
                ))
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_entities_controlled_by_factor_source(
        &self,
        factor_source: FactorSource,
        is_mnemonic_present_in_keychain: bool,
        is_mnemonic_marked_as_backed_up: bool,
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> EntitiesControlledByFactorSource {
        let accounts = accounts
            .iter()
            .filter(|a| {
                a.security_state
                    .is_controlled_by_factor_source(factor_source.clone())
            })
            .collect();
        let hidden_accounts = hidden_accounts
            .iter()
            .filter(|a| {
                a.security_state
                    .is_controlled_by_factor_source(factor_source.clone())
            })
            .collect();
        let personas = personas
            .iter()
            .filter(|p| {
                p.security_state
                    .is_controlled_by_factor_source(factor_source.clone())
            })
            .collect();
        let hidden_personas = hidden_personas
            .iter()
            .filter(|p| {
                p.security_state
                    .is_controlled_by_factor_source(factor_source.clone())
            })
            .collect();
        EntitiesControlledByFactorSource {
            factor_source,
            is_mnemonic_present_in_keychain,
            is_mnemonic_marked_as_backed_up,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }

    async fn is_mnemonic_present_in_keychain(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        match factor_source {
            FactorSource::Device { value } => {
                self.clients
                    .secure_storage
                    .contains_device_mnemonic(value)
                    .await
            }
            _ => Err(CommonError::Unknown),
        }
    }

    async fn is_mnemonic_marked_as_backed_up(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        match factor_source {
            FactorSource::Device { value } => {
                self.clients
                    .unsafe_storage
                    .check_if_mnemonic_is_backed_up(value)
                    .await
            }
            _ => Err(CommonError::Unknown),
        }
    }
}
