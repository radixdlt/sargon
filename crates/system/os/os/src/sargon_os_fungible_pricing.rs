use crate::prelude::*;

impl SargonOS {
    /// Fetches scoped fiat prices for fungible tokens and LSUs.
    ///
    /// The request is scoped by the provided sets of resource addresses and
    /// cached for 5 minutes.
    pub async fn fetch_fungible_fiat_values(
        &self,
        tokens: HashSet<ResourceAddress>,
        lsus: HashSet<ResourceAddress>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<HashMap<ResourceAddress, Decimal192>> {
        let token_price_services = self
            .profile_state_holder
            .token_price_services_on_current_network()?;
        self.nft_prices_client
            .fetch_fungible_fiat_values_using_token_price_services(
                tokens,
                lsus,
                currency,
                token_price_services,
                force_fetch,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct TokenPriceResponseBody {
        tokens: Vec<TokenPriceResponseItem>,
        lsus: Vec<TokenPriceResponseItem>,
    }

    #[derive(serde::Serialize)]
    struct TokenPriceResponseItem {
        resource_address: ResourceAddress,
        usd_price: f64,
    }

    #[actix_rt::test]
    async fn fetch_fungible_fiat_values_uses_current_network_services_with_failover(
    ) {
        let captured_requests =
            Arc::new(std::sync::Mutex::new(Vec::<NetworkRequest>::new()));
        let captured_requests_clone = captured_requests.clone();
        let token = ResourceAddress::sample_mainnet_xrd();

        let success_response = TokenPriceResponseBody {
            tokens: vec![TokenPriceResponseItem {
                resource_address: token.clone(),
                usd_price: 1.0,
            }],
            lsus: vec![],
        };
        let success_body = serde_json::to_vec(&success_response).unwrap();

        let networking = Arc::new(MockNetworkingDriver::with_lazy_responses(
            move |request, _| {
                if request.url.path() != "/price/tokens" {
                    return NetworkResponse::new(500, vec![]);
                }

                captured_requests_clone
                    .lock()
                    .unwrap()
                    .push(request.clone());

                if request.url.host_str()
                    == Some("token-prices-secondary.example")
                {
                    NetworkResponse::new(200, success_body.clone())
                } else {
                    NetworkResponse::new(500, vec![])
                }
            },
        ));

        let os = SUT::boot_test_with_networking_driver(networking)
            .await
            .unwrap();
        os.with_timeout(|x| x.change_current_gateway(Gateway::stokenet()))
            .await
            .unwrap();

        os.with_timeout(|x| {
            x.add_token_price_service_on_current_network(
                Url::parse("https://token-prices-primary.example").unwrap(),
            )
        })
        .await
        .unwrap();
        os.with_timeout(|x| {
            x.add_token_price_service_on_current_network(
                Url::parse("https://token-prices-secondary.example").unwrap(),
            )
        })
        .await
        .unwrap();

        let prices = os
            .with_timeout(|x| {
                x.fetch_fungible_fiat_values(
                    HashSet::from([token.clone()]),
                    HashSet::new(),
                    FiatCurrency::USD,
                    true,
                )
            })
            .await
            .unwrap();

        assert_eq!(prices.get(&token), Some(&Decimal192::one()));

        let captured = captured_requests.lock().unwrap();
        let attempted_hosts: Vec<String> = captured
            .iter()
            .filter_map(|r| r.url.host_str().map(|h| h.to_string()))
            .collect();
        assert_eq!(
            attempted_hosts,
            vec![
                "token-price-service.radixdlt.com".to_owned(),
                "token-prices-primary.example".to_owned(),
                "token-prices-secondary.example".to_owned(),
            ]
        );
    }
}
