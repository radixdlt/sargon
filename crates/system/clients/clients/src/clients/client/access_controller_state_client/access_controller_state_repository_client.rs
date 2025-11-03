pub use crate::prelude::*;

pub struct AccessControllerStateRepositoryClient {
    pub(crate) http_client: HttpClient,
    pub(crate) cache_client: AccessControllerDetailsCacheClient,
}

impl AccessControllerStateRepositoryClient {
    pub fn new(
        http_client: HttpClient,
        cache_client: AccessControllerDetailsCacheClient,
    ) -> Self {
        Self {
            http_client,
            cache_client,
        }
    }
}

impl AccessControllerStateRepositoryClient {
    pub async fn fetch_access_controllers_details(
        &self,
        addresses: Vec<AccessControllerAddress>,
        network_id: NetworkID,
    ) -> Result<Vec<AccessControllerStateDetails>> {
        let gateway_client = GatewayClient::with_http_client(
            self.http_client.clone(),
            network_id,
        );
        let details = gateway_client
            .fetch_access_controllers_details(addresses.clone())
            .await?;
        self.cache_client.insert_many(details.clone()).await?;
        Ok(details)
    }

    pub async fn get_cached_access_controller_details(
        &self,
        address: &AccessControllerAddress,
    ) -> Result<AccessControllerStateDetails> {
        let cache = self.cache_client.snapshot().await?;
        cache.get(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerStateRepositoryClient;

    #[actix_rt::test]
    async fn fetch_access_controllers_details_returns_details_for_valid_addresses(
    ) {
        let mock_networking_driver =
            MockNetworkingDriver::new_with_responses(vec![
                gateway_status_response(),
                state_entity_details_response(),
            ]);
        let http_client = HttpClient::new(Arc::new(mock_networking_driver));
        let file_system = Arc::new(FileSystemClient::in_memory());
        let cache_client = AccessControllerDetailsCacheClient::new(file_system);
        let sut = SUT::new(http_client, cache_client);

        let addresses = vec![
            AccessControllerAddress::sample(),
            AccessControllerAddress::sample_other(),
        ];
        let network_id = NetworkID::Mainnet;
        let expected_details = vec![
            AccessControllerStateDetails::sample(),
            AccessControllerStateDetails::sample_other(),
        ];

        let result = sut
            .fetch_access_controllers_details(addresses, network_id)
            .await;

        pretty_assertions::assert_eq!(result.unwrap(), expected_details);
    }

    /// Creates a mock response for `GatewayStatusResponse`.
    fn gateway_status_response() -> MockNetworkingDriverResponse {
        MockNetworkingDriverResponse::new_success(GatewayStatusResponse {
            ledger_state: LedgerState::sample(),
        })
    }

    /// Creates a mock response for `StateEntityDetailsResponse`.
    fn state_entity_details_response() -> MockNetworkingDriverResponse {
        let items: Vec<StateEntityDetailsResponseItem> = vec![
            StateEntityDetailsResponseItem::new(
                Address::AccessController(AccessControllerAddress::sample()),
                None,
                None,
                EntityMetadataCollection::empty(),
                Some(StateEntityDetailsResponseItemDetails::Component(
                    StateEntityDetailsResponseComponentDetails {
                        role_assignments: None,
                        state: Some(
                            serde_json::to_string(
                                &AccessControllerFieldStateValue::sample(),
                            )
                            .unwrap(),
                        ),
                    },
                )),
            ),
            StateEntityDetailsResponseItem::new(
                Address::AccessController(
                    AccessControllerAddress::sample_other(),
                ),
                None,
                None, // non-fungible
                EntityMetadataCollection::empty(),
                Some(StateEntityDetailsResponseItemDetails::Component(
                    StateEntityDetailsResponseComponentDetails {
                        role_assignments: None,
                        state: Some(
                            serde_json::to_string(
                                &AccessControllerFieldStateValue::sample_other(
                                ),
                            )
                            .unwrap(),
                        ),
                    },
                )),
            ),
        ];
        MockNetworkingDriverResponse::new_success(
            StateEntityDetailsResponse::new(LedgerState::sample(), items),
        )
    }
}
