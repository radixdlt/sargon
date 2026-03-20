use crate::prelude::*;
use sargon::SargonOS as InternalSargonOS;

#[uniffi::export]
impl SargonOS {
    /// Fetches scoped fiat prices for fungible tokens and LSUs.
    pub async fn fetch_fungible_fiat_values(
        &self,
        tokens: Vec<ResourceAddress>,
        lsus: Vec<ResourceAddress>,
        currency: FiatCurrency,
        force_fetch: bool,
    ) -> Result<HashMap<ResourceAddress, Decimal192>> {
        let tokens_set: HashSet<sargon::ResourceAddress> = tokens
            .into_iter()
            .map(|address| address.into_internal())
            .collect();
        let lsus_set: HashSet<sargon::ResourceAddress> = lsus
            .into_iter()
            .map(|address| address.into_internal())
            .collect();

        let result = self
            .wrapped
            .fetch_fungible_fiat_values(
                tokens_set,
                lsus_set,
                currency.into_internal(),
                force_fetch,
            )
            .await?;

        Ok(result
            .into_iter()
            .map(|(resource_address, price)| {
                (resource_address.into(), price.into())
            })
            .collect())
    }
}
