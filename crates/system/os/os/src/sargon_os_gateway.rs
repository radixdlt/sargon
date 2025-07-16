use crate::prelude::*;

// ==================
// Get Current Gateway/Network
// ==================
impl SargonOS {
    /// Returns the id of the Network of the current Gateway set in AppPreferences
    /// of the active Profile. This is the canonical value of "current network",
    /// which affects which accounts host clients display to end user and to
    /// which network transactions are submitted, amongst other behaviors.
    pub fn current_network_id(&self) -> Result<NetworkID> {
        self.profile_state_holder.current_network_id()
    }

    /// The current gateway host client is using, which affects `current_network_id`.
    /// All Network Requests reading from Radix ledger and submission of new
    /// transactions will go the the Radix Network of the current Gateway.
    pub fn current_gateway(&self) -> Result<Gateway> {
        self.profile_state_holder.current_gateway()
    }

    /// Returns the `gateways` values of the current Profile.
    pub fn gateways(&self) -> Result<SavedGateways> {
        self.profile_state_holder.gateways()
    }

    /// Returns the `ProfileNetwork` corresponding to the network ID set by the
    /// current gateway.
    pub fn current_network(&self) -> Result<ProfileNetwork> {
        self.profile_state_holder.current_network()
    }
}

// ==================
// Change Current Gateway
// ==================
impl SargonOS {
    /// Changes the current Gateway to `to`, if it is not already the current.
    /// Returns the outcome of the change, if we did in fact switch (current != to),
    /// and if we switched then if `to` is new.
    ///
    /// If we did in fact change current, an `EventNotification` is emitted.
    ///
    /// # Emits Event
    /// Emits `Event::GatewayChangedCurrent` if we changed the gateway.
    pub async fn change_current_gateway(
        &self,
        to: Gateway,
    ) -> Result<ChangeGatewayOutcome> {
        info!("Changing current gateway to: {}", &to);
        let network_id = to.network.id;
        let outcome = self
            .update_profile_with(|p| {
                let outcome =
                    p.app_preferences.gateways.change_current(to.clone());
                match outcome {
                    ChangeGatewayOutcome::DidChange { is_new: _ } => {
                        if !p.networks.contains_id(network_id) {
                            p.networks.append(ProfileNetwork::new_empty_on(
                                network_id,
                            ));
                        }
                        Ok(outcome)
                    }
                    ChangeGatewayOutcome::NoChange => Ok(outcome),
                }
            })
            .await?;

        match outcome {
            ChangeGatewayOutcome::DidChange { is_new } => {
                self.event_bus
                    .emit(EventNotification::new(
                        Event::GatewayChangedCurrent { to, is_new },
                    ))
                    .await;
            }
            ChangeGatewayOutcome::NoChange => {}
        };

        debug!("Change current gateway outcome: {:?}", &outcome);

        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_change_gateway_creates_empty_network_if_needed() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let number_of_networks_before_change =
            os.profile().unwrap().networks.len();

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().unwrap().networks.len(),
            number_of_networks_before_change + 1
        );
    }

    #[actix_rt::test]
    async fn test_change_gateway_gateways_returns_updated() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.gateways().unwrap().current, Gateway::stokenet())
    }

    #[actix_rt::test]
    async fn test_change_gateway_current_returns_updated() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.current_gateway().unwrap(), Gateway::stokenet())
    }

    #[actix_rt::test]
    async fn test_change_gateway_current_returns_updated_network_id() {
        // ARRANGE
        let os = SUT::fast_boot().await;

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(os.current_network_id().unwrap(), NetworkID::Stokenet)
    }

    #[actix_rt::test]
    async fn test_change_gateway_emits_event() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert!(event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::GatewayChangedCurrent));
    }

    #[actix_rt::test]
    async fn test_change_to_current_gateway_does_not_emits_event() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let mut clients = Clients::new(Bios::new(drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let os = timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SUT::boot_with_clients_and_interactor(clients, interactors),
        )
        .await
        .unwrap();
        os.with_timeout(|x| x.new_wallet()).await.unwrap();

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::mainnet()))
            .await
            .unwrap();

        // ASSERT
        assert!(!event_bus_driver
            .recorded()
            .iter()
            .any(|e| e.event.kind() == EventKind::GatewayChangedCurrent));
    }
}
