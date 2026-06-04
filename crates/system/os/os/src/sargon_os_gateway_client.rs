use crate::prelude::*;

// ==================
// Get an instance of the GatewayClient
// ==================
impl SargonOS {
    /// Returns a new instance if the `GatewayClient` on the provided `network_id`.
    pub fn gateway_client_with(&self, network_id: NetworkID) -> GatewayClient {
        let gateway = self
            .gateways()
            .ok()
            .and_then(|gateways| {
                gateways
                    .all()
                    .into_iter()
                    .find(|gateway| gateway.network.id == network_id)
            })
            .unwrap_or_else(|| Gateway::from(network_id));

        GatewayClient::with_gateway(self.http_client.driver.clone(), gateway)
    }

    /// Returns a new instance if the `GatewayClient` on the current network's
    /// gateway alongside the network id as a tuple.
    pub fn gateway_client_on(&self) -> Result<(GatewayClient, NetworkID)> {
        let gateway = self.current_gateway()?;
        let network_id = gateway.network.id;
        let client = GatewayClient::with_gateway(
            self.http_client.driver.clone(),
            gateway,
        );

        Ok((client, network_id))
    }

    /// Returns a new instance if the `GatewayClient` on the current network's
    /// gateway.
    pub fn gateway_client(&self) -> Result<GatewayClient> {
        self.gateway_client_on().map(|(client, _)| client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    pub async fn gateway_client_is_in_correct_network() {
        // ARRANGE
        let sut = SUT::fast_boot().await;

        // ACT
        let client = sut.gateway_client().unwrap();

        // ASSERT
        assert_eq!(client.gateway.network.id, NetworkID::Mainnet)
    }

    #[actix_rt::test]
    pub async fn gateway_client_on_network_id_matches_gateway_id() {
        // ARRANGE
        let sut = SUT::fast_boot().await;

        // ACT
        let (_, network_id) = sut.gateway_client_on().unwrap();

        // ASSERT
        assert_eq!(network_id, NetworkID::Mainnet)
    }

    #[actix_rt::test]
    pub async fn gateway_client_is_in_correct_network_when_changing_gateway_in_profile(
    ) {
        // ARRANGE
        let sut = SUT::fast_boot().await;
        sut.change_current_gateway(Gateway::stokenet())
            .await
            .unwrap();

        // ACT
        let client = sut.gateway_client().unwrap();

        // ASSERT
        assert_eq!(client.gateway.network.id, NetworkID::Stokenet)
    }

    #[actix_rt::test]
    pub async fn gateway_client_uses_current_custom_gateway_url() {
        // ARRANGE
        let sut = SUT::fast_boot().await;
        let custom_gateway = Gateway::new(
            "https://custom-mainnet.example/".to_string(),
            NetworkID::Mainnet,
        )
        .unwrap();
        sut.change_current_gateway(custom_gateway.clone())
            .await
            .unwrap();

        // ACT
        let client = sut.gateway_client().unwrap();

        // ASSERT
        assert_eq!(client.gateway, custom_gateway)
    }

    #[actix_rt::test]
    pub async fn gateway_client_with_uses_current_custom_gateway_for_network() {
        // ARRANGE
        let sut = SUT::fast_boot().await;
        let custom_gateway = Gateway::new(
            "https://custom-stokenet.example/".to_string(),
            NetworkID::Stokenet,
        )
        .unwrap();
        sut.change_current_gateway(custom_gateway.clone())
            .await
            .unwrap();

        // ACT
        let client = sut.gateway_client_with(NetworkID::Stokenet);

        // ASSERT
        assert_eq!(client.gateway, custom_gateway)
    }

    #[actix_rt::test]
    pub async fn error_is_returned_when_profile_is_not_present() {
        // ARRANGE
        let test_drivers = Drivers::test();
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();

        let interactors = Interactors::new_from_clients(&clients);
        let sut =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        // ACT
        let res = sut.gateway_client();

        // ASSERT
        assert!(matches!(
            res,
            Err(CommonError::ProfileStateNotLoaded { .. })
        ))
    }
}
