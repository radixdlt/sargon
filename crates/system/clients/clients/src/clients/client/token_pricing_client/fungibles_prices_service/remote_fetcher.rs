use crate::prelude::*;

/// The URL endpoint for the Radix token price service scoped API.
pub const FETCH_URL: &str =
    "https://token-price-service.radixdlt.com/price/tokens";

impl FungiblesPricesClient {
    pub async fn fetch_remote_token_prices(
        &self,
        request_body: &FungiblePricesRequest,
    ) -> Result<PerTokenPrices> {
        let request =
            NetworkRequest::new_post(Url::from_str(FETCH_URL).unwrap())
                .with_serializing_body(request_body.clone())?
                .with_gateway_api_headers();

        let response: ScopedTokenPricesResponse = self
            .http_client
            .execute_request_with_decoding(request)
            .await?;

        Ok(response.into_prices_map())
    }
}
