use crate::prelude::*;

impl SargonOS {
    /// Returns token price services configured on the current network.
    pub fn token_price_services_on_current_network(
        &self,
    ) -> Result<TokenPriceServices> {
        self.profile_state_holder
            .token_price_services_on_current_network()
    }
}

impl SargonOS {
    /// Adds a token price service endpoint on the current network.
    /// Returns false if endpoint already exists.
    pub async fn add_token_price_service_on_current_network(
        &self,
        base_url: Url,
    ) -> Result<bool> {
        self.update_profile_with(|profile| {
            let current_network = profile.current_network_id();
            let mut did_add = false;
            profile.networks.update_with(current_network, |network| {
                did_add = network
                    .token_price_services
                    .add(TokenPriceService::new(base_url.clone()));
            });
            Ok(did_add)
        })
        .await
    }

    /// Deletes a token price service endpoint on the current network.
    /// Returns false if endpoint is missing or if this is the last endpoint.
    pub async fn delete_token_price_service_on_current_network(
        &self,
        base_url: Url,
    ) -> Result<bool> {
        self.update_profile_with(|profile| {
            let current_network = profile.current_network_id();
            let mut did_delete = false;
            profile.networks.update_with(current_network, |network| {
                did_delete =
                    network.token_price_services.remove_by_base_url(&base_url);
            });
            Ok(did_delete)
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn token_price_services_default_on_current_network() {
        let os = SUT::fast_boot().await;
        let services = os.token_price_services_on_current_network().unwrap();
        assert_eq!(services, TokenPriceServices::default());
    }

    #[actix_rt::test]
    async fn add_token_price_service_on_current_network() {
        let os = SUT::fast_boot().await;
        let url =
            Url::parse("https://token-price-service-alt.example").unwrap();

        let did_add = os
            .with_timeout(|x| {
                x.add_token_price_service_on_current_network(url.clone())
            })
            .await
            .unwrap();

        assert!(did_add);
        let stored = os.token_price_services_on_current_network().unwrap();
        assert!(stored.contains_id(url));
    }

    #[actix_rt::test]
    async fn add_duplicate_token_price_service_returns_false() {
        let os = SUT::fast_boot().await;
        let url =
            Url::parse("https://token-price-service-alt.example").unwrap();

        _ = os
            .with_timeout(|x| {
                x.add_token_price_service_on_current_network(url.clone())
            })
            .await
            .unwrap();
        let did_add_again = os
            .with_timeout(|x| {
                x.add_token_price_service_on_current_network(url.clone())
            })
            .await
            .unwrap();

        assert!(!did_add_again);
    }

    #[actix_rt::test]
    async fn delete_missing_or_last_token_price_service_returns_false() {
        let os = SUT::fast_boot().await;
        let missing =
            Url::parse("https://missing-token-price-service.example").unwrap();

        let missing_deleted = os
            .with_timeout(|x| {
                x.delete_token_price_service_on_current_network(missing.clone())
            })
            .await
            .unwrap();
        assert!(!missing_deleted);

        let only_default = TokenPriceService::production().base_url;
        let deleted_last = os
            .with_timeout(|x| {
                x.delete_token_price_service_on_current_network(
                    only_default.clone(),
                )
            })
            .await
            .unwrap();
        assert!(!deleted_last);
    }

    #[actix_rt::test]
    async fn delete_token_price_service_when_more_than_one_returns_true() {
        let os = SUT::fast_boot().await;
        let extra =
            Url::parse("https://token-price-service-alt.example").unwrap();

        os.with_timeout(|x| {
            x.add_token_price_service_on_current_network(extra.clone())
        })
        .await
        .unwrap();

        let did_delete = os
            .with_timeout(|x| {
                x.delete_token_price_service_on_current_network(extra.clone())
            })
            .await
            .unwrap();

        assert!(did_delete);
        let stored = os.token_price_services_on_current_network().unwrap();
        assert!(!stored.contains_id(extra));
    }
}
