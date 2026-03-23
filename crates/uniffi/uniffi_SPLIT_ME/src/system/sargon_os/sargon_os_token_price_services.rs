use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub fn token_price_services_on_current_network(
        &self,
    ) -> Result<Vec<TokenPriceService>> {
        let services: sargon::TokenPriceServices = self
            .wrapped
            .token_price_services_on_current_network()
            .into_result()?;
        Ok(services.into_iter().map(Into::into).collect())
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn add_token_price_service_on_current_network(
        &self,
        base_url: Url,
    ) -> Result<bool> {
        self.wrapped
            .add_token_price_service_on_current_network(base_url)
            .await
            .into_result()
    }

    pub async fn delete_token_price_service_on_current_network(
        &self,
        base_url: Url,
    ) -> Result<bool> {
        self.wrapped
            .delete_token_price_service_on_current_network(base_url)
            .await
            .into_result()
    }
}
