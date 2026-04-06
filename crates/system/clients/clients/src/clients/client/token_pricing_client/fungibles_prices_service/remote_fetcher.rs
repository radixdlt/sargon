use crate::prelude::*;

impl FungiblesPricesClient {
    pub async fn fetch_remote_token_prices(
        &self,
        request_body: &FungiblePricesRequest,
    ) -> Result<PerTokenPrices> {
        self.fetch_remote_token_prices_using_token_price_services(
            request_body,
            TokenPriceServices::default(),
        )
        .await
    }

    pub async fn fetch_remote_token_prices_using_token_price_services(
        &self,
        request_body: &FungiblePricesRequest,
        token_price_services: TokenPriceServices,
    ) -> Result<PerTokenPrices> {
        if token_price_services.is_empty() {
            return Err(CommonError::ExpectedNonEmptyCollection);
        }

        let mut last_error: Option<CommonError> = None;

        for service in token_price_services {
            let url = match service.scoped_tokens_url() {
                Ok(url) => url,
                Err(error) => {
                    last_error = Some(error);
                    continue;
                }
            };
            match self
                .fetch_remote_token_prices_from_url(request_body, url)
                .await
            {
                Ok(prices) => return Ok(prices),
                Err(error) => {
                    last_error = Some(error);
                }
            }
        }

        Err(last_error.unwrap_or(CommonError::ExpectedNonEmptyCollection))
    }

    async fn fetch_remote_token_prices_from_url(
        &self,
        request_body: &FungiblePricesRequest,
        fetch_url: Url,
    ) -> Result<PerTokenPrices> {
        let request = NetworkRequest::new_post(fetch_url)
            .with_serializing_body(request_body.clone())?
            .with_gateway_api_headers();

        let response: ScopedTokenPricesResponse = self
            .http_client
            .execute_request_with_decoding(request)
            .await?;

        Ok(response.into_prices_map())
    }
}
