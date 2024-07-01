use crate::prelude::*;

#[uniffi::export]
impl GatewayClient {
    /// Fetched the XRD balance of account of `address`, returns `None` if
    /// it has no balance.
    pub async fn xrd_balance_of_account(
        &self,
        address: AccountAddress,
    ) -> Result<Option<Decimal192>> {
        let response: StateEntityDetailsResponse = self
            .state_entity_details(StateEntityDetailsRequest::new(vec![
                address.address()
            ]))
            .await?;

        let Some(response_item) = response
            .items
            .into_iter()
            .find(|x| x.address == address.into())
        else {
            return Ok(None);
        };

        let fungible_resources = response_item
            .fungible_resources
            .expect("Never None for Account");

        let xrd_address = ResourceAddress::xrd_on_network(address.network_id());

        let Some(xrd_resource_collection_item) = fungible_resources
            .items
            .into_iter()
            .find(|x| x.resource_address() == xrd_address)
        else {
            return Ok(None);
        };

        let xrd_resource = xrd_resource_collection_item
            .as_global()
            .expect("Global is default");

        Ok(Some(xrd_resource.amount))
    }

    /// Fetched the XRD balance of account of `address`, returns `0` if
    /// it has no balance.
    pub async fn xrd_balance_of_account_or_zero(
        &self,
        address: AccountAddress,
    ) -> Result<Decimal192> {
        self.xrd_balance_of_account(address)
            .await
            .map(|x| x.unwrap_or(Decimal192::zero()))
    }
}

impl GatewayClient {
    /// Fetches the metadata for the given address and returns the `icon_url` value.
    pub async fn icon_url_of_address(
        &self,
        address: String,
    ) -> Result<Option<Url>> {
        let response: StateEntityDetailsResponse = self
            .state_entity_details(StateEntityDetailsRequest::address(
                address.clone(),
                vec![MetadataKey::IconUrl],
            ))
            .await?;

        let Some(response_item) = response
            .items
            .into_iter()
            .find(|x| x.address.to_string() == address)
        else {
            return Ok(None);
        };

        let Some(item) = response_item
            .metadata
            .items
            .into_iter()
            .find(|x| x.key == MetadataKey::IconUrl.to_string())
        else {
            return Ok(None);
        };

        match item.value.typed {
            MetadataTypedValue::MetadataUrlValue { value } => Ok(Some(value)),
            _ => Err(CommonError::InvalidMetadataFormat),
        }
    }
}
