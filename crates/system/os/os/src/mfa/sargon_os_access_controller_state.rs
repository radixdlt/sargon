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
                    .map(|s| s.access_controller_address.clone())
            })
            .collect()
    }
}
