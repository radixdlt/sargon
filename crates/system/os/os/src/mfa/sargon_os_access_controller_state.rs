use crate::prelude::*;

impl SargonOS {
    pub async fn fetch_all_access_controllers_details(
        &self,
    ) -> Result<Vec<AccessControllerStateDetails>> {
        let network_id = self.profile_state_holder.current_network_id()?;
        let accounts =
            self.profile_state_holder.accounts_on_current_network()?;
        let personas =
            self.profile_state_holder.personas_on_current_network()?;

        let mut ac_addresses =
            Self::extract_access_controller_addresses(accounts);
        ac_addresses
            .extend(Self::extract_access_controller_addresses(personas));

        self.clients
            .access_controller_state_repository_client
            .fetch_access_controllers_details(ac_addresses, network_id)
            .await
    }

    pub async fn is_recovery_proposal_unknown(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<bool> {
        let entity =
            self.profile_state_holder.entity_by_address(entity_address);
        let security_state = entity?
            .security_state()
            .as_securified()
            .expect("Entity must be securified")
            .clone();
        let Some(provisional_config) =
            security_state.provisional_securified_config
        else {
            // The proposal is unknown if the entity has no provisional config
            return Ok(true);
        };

        let ac_state_details = self
            .access_controller_state_repository_client
            .get_cached_access_controller_details(
                &security_state.access_controller_address,
            )
            .await?;
        let Some(recovery_attempt) =
            ac_state_details.state.recovery_role_recovery_attempt
        else {
            return Err(CommonError::AccessControllerNotInRecoveryState);
        };

        let provisional_config_rule_set = ScryptoRuleSet::from(
            provisional_config
                .get_security_structure_of_factor_instances()
                .matrix_of_factors,
        );
        let on_ledger_rule_set =
            ScryptoRuleSet::from(recovery_attempt.recovery_proposal);

        Ok(provisional_config_rule_set != on_ledger_rule_set)
    }

    fn extract_access_controller_addresses<I, E>(
        entities: I,
    ) -> Vec<AccessControllerAddress>
    where
        I: IntoIterator<Item = E>,
        E: HasSecurityState,
    {
        entities
            .into_iter()
            .filter_map(|entity| {
                entity
                    .security_state()
                    .as_securified()
                    .map(|s| s.access_controller_address)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;
}
