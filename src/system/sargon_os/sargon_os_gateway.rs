use crate::prelude::*;

// ==================
// Get Current Gateway/Network
// ==================
#[uniffi::export]
impl SargonOS {
    pub fn current_network_id(&self) -> NetworkID {
        self.profile_holder.current_network_id()
    }

    pub fn current_gateway(&self) -> Gateway {
        self.profile_holder.current_gateway().clone()
    }

    pub fn gateways(&self) -> SavedGateways {
        self.profile_holder.gateways().clone()
    }

    pub fn current_network(&self) -> ProfileNetwork {
        self.profile_holder.current_network().clone()
    }
}

// ==================
// Change Current Gateway
// ==================
#[uniffi::export]
impl SargonOS {
    /// Changes the current Gateway to `to`, if it is not already the current.
    /// Returns the outcome of the change, if we did in fact switch (current != to),
    /// and if we switched then if `to` as new.
    ///
    /// If we did in fact change current, an `EventNotification` is emitted.
    pub async fn change_current_gateway(
        &self,
        to: Gateway,
    ) -> Result<ChangeGatewayOutcome> {
        let network_id = to.network.id;
        let outcome = self
            .update_profile_with(|mut p| {
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
        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::{future::Future, time::Duration};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_change_gateway_creates_empty_network_if_needed() {
        // ARRANGE
        let os = SUT::fast_boot().await;
        let number_of_networks_before_change = os.profile().networks.len();

        // ACT
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        // ASSERT
        assert_eq!(
            os.profile().networks.len(),
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
        assert_eq!(os.gateways().current, Gateway::stokenet())
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
        assert_eq!(os.current_gateway(), Gateway::stokenet())
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
        assert_eq!(os.current_network_id(), NetworkID::Stokenet)
    }

    #[actix_rt::test]
    async fn test_change_gateway_emits_event() {
        // ARRANGE (and ACT)
        let event_bus_driver = RustEventBusDriver::new();
        let drivers = Drivers::with_event_bus(event_bus_driver.clone());
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

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
        let bios = Bios::new(drivers);

        let os = timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, SUT::boot(bios))
            .await
            .unwrap()
            .unwrap();

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
