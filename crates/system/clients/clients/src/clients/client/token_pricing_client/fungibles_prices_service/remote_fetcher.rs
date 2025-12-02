use crate::prelude::*;

/// The URL endpoint for the Radix token price service API
pub const FETCH_URL: &str = "https://token-price-service.radixdlt.com/tokens";

impl FungiblesPricesClient {
    pub async fn fetch_remote_token_prices(&self) -> Result<Vec<TokenPrice>> {
        let request =
            NetworkRequest::new_post(Url::from_str(FETCH_URL).unwrap());
        self.http_client
            .execute_request_with_decoding(request)
            .await
    }
}
