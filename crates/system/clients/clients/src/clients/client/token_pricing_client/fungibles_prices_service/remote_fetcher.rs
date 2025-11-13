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

/// Represents a token's price in a specific fiat currency.
///
/// This structure is returned by the remote token pricing API and used internally
/// for price calculations. The price is stored as f32 from the API but converted
/// to Decimal192 for precise calculations.
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct TokenPrice {
    /// The on-ledger address of the token resource
    pub resource_address: ResourceAddress,
    /// The price per unit of the token in the specified currency
    pub price: f32,
    /// The fiat currency this price is denominated in
    pub currency: FiatCurrency,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decoding() {
        let raw_json = r#"
        [
  {
    "id": 2,
    "resource_address": "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
    "symbol": "$BOBBY",
    "name": "Bobby",
    "price": 0.04134813463690507,
    "currency": "USD"
  },
  {
    "id": 102,
    "resource_address": "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
    "symbol": "$MRD",
    "name": "Memerad",
    "price": 0.000004698356561816775,
    "currency": "USD"
  },
  {
    "id": 133,
    "resource_address": "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
    "symbol": "$WOWO",
    "name": "WOWO",
    "price": 0.00007746121946503883,
    "currency": "USD"
  }
  ]
        "#;

        let decoded: Vec<TokenPrice> = serde_json::from_str(raw_json).unwrap();
        let expected_tokens = vec![
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t45js47zxtau85v0tlyayerzrgfpmguftlfwfr5fxzu42qtu72tnt0",
                )
                .unwrap(),
                price: 0.04134813463690507,
                currency: FiatCurrency::USD,
            },
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t5u04cs3u2yxqkcwku7jdvdvv9cu739jsx0rdwu97682lr0rn92qdh",
                )
                .unwrap(),
                price: 0.000004698356561816775,
                currency: FiatCurrency::USD,
            },
            TokenPrice {
                resource_address: ResourceAddress::from_str(
                    "resource_rdx1t4kc5ljyrwlxvg54s6gnctt7nwwgx89h9r2gvrpm369s23yhzyyzlx",
                )
                .unwrap(),
                price: 0.00007746121946503883,
                currency: FiatCurrency::USD,
            },
        ];

        pretty_assertions::assert_eq!(decoded, expected_tokens);
    }
}
