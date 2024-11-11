use crate::prelude::*;

// ==================
// Delete Account
// ==================
impl SargonOS {
    pub async fn create_delete_account_manifest(
        &self,
        account_address: AccountAddress,
    ) -> Result<TransactionManifest> {
        let network_id = account_address.network_id();
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        let resource_preferences = self
            .get_account_resource_preferences(gateway_client, account_address)
            .await?;

        let manifest = TransactionManifest::delete_account(
            &account_address,
            resource_preferences,
        );
        Ok(manifest)
    }
}

impl SargonOS {
    async fn get_account_resource_preferences(
        &self,
        gateway_client: GatewayClient,
        account_address: AccountAddress,
    ) -> Result<Vec<AccountResourcePreferencesResponseItem>> {
        let mut items: Vec<AccountResourcePreferencesResponseItem> = Vec::new();
        let mut more_to_load = true;
        let mut cursor: Option<String> = None;
        while (more_to_load) {
            let request = AccountResourcePreferencesRequest::new(
                account_address,
                cursor.clone(),
                GATEWAY_PAGE_REQUEST_LIMIT,
            );
            let response =
                gateway_client.account_resource_preferences(request).await?;
            items.extend(response.items);
            cursor = response.next_cursor;
            more_to_load = cursor.is_some();
        }

        Ok(items)
    }
}
