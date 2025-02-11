use crate::prelude::*;

pub trait IntoNetworkResponse {
    fn into_network_response(self) -> NetworkResponse;
}

impl<T: Serialize> IntoNetworkResponse for T {
    fn into_network_response(self) -> NetworkResponse {
        NetworkResponse::new(200, serde_json::to_vec(&self).unwrap())
    }
}

pub trait NetworkRequestParseOriginal {
    fn parse_original<T: for<'a> Deserialize<'a>>(&self) -> T;
}

impl NetworkRequestParseOriginal for NetworkRequest {
    fn parse_original<T: for<'a> Deserialize<'a>>(&self) -> T {
        serde_json::from_slice(&self.body).unwrap()
    }
}

pub trait EveryoneIsRichMockNetworkingDriver {
    fn everyones_rich(network_id: NetworkID) -> Arc<dyn NetworkingDriver>;
}

impl EveryoneIsRichMockNetworkingDriver for MockNetworkingDriver {
    fn everyones_rich(network_id: NetworkID) -> Arc<dyn NetworkingDriver> {
        mock_networking_driver_balance(network_id, |_| {
            Decimal::from(1_000_000_000)
        })
    }
}

pub trait EveryoneIsBrokeMockNetworkingDriver {
    fn everyones_broke(network_id: NetworkID) -> Arc<dyn NetworkingDriver>;
}

impl EveryoneIsBrokeMockNetworkingDriver for MockNetworkingDriver {
    fn everyones_broke(network_id: NetworkID) -> Arc<dyn NetworkingDriver> {
        mock_networking_driver_balance(network_id, |_| Decimal::zero())
    }
}

pub(crate) fn mock_networking_driver_balance(
    network_id: NetworkID,
    balance_of: impl Fn(&Address) -> Decimal + Sync + Send + 'static,
) -> Arc<dyn NetworkingDriver> {
    Arc::new(MockNetworkingDriver::with_lazy_responses(
        move |req: NetworkRequest, _: u64| -> NetworkResponse {
            let path = req.url.path();
            if path.ends_with(GatewayClient::PATH_STATE_ENTITY_DETAILS) {
                let request = req.parse_original::<StateEntityDetailsRequest>();

                StateEntityDetailsResponse::new(
                    None,
                    request.addresses.iter().map(|address| {
                        let balance = balance_of(address);
                        let items =
                            vec![FungibleResourcesCollectionItem::global(
                                ResourceAddress::xrd_on_network(network_id),
                                balance,
                            )];
                        StateEntityDetailsResponseItem::new(
                            *address,
                            FungibleResourcesCollection::new(None, None, items),
                            None, // non-fungible
                            EntityMetadataCollection::empty(),
                            None, // details
                        )
                    }),
                )
                .into_network_response()
            } else if path
                .ends_with(GatewayClient::PATH_TRANSACTION_CONSTRUCTION)
            {
                TransactionConstructionResponse::new(LedgerState::new(
                    NetworkID::Mainnet.logical_name(),
                    1,
                    "2021-01-01T00:00:00Z",
                    1, // epoch
                    1,
                ))
                .into_network_response()
            } else {
                todo!("Unimplemented mock data for path: {:?}", path)
            }
        },
    ))
}
