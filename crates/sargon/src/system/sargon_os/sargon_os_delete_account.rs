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
        let resource_preferences = self
            .get_account_resource_preferences(network_id, account_address)
            .await?;

        let authorized_depositors = self
            .get_account_authorized_depositors(network_id, account_address)
            .await?;

        let manifest = TransactionManifest::delete_account(
            &account_address,
            resource_preferences,
            authorized_depositors,
        )?;

        Ok(manifest)
    }
}

impl SargonOS {
    async fn get_account_resource_preferences(
        &self,
        network_id: NetworkID,
        account_address: AccountAddress,
    ) -> Result<Vec<AccountResourcePreferencesResponseItem>> {
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let mut items: Vec<AccountResourcePreferencesResponseItem> = Vec::new();
        let mut more_to_load = true;
        let mut cursor: Option<String> = None;
        while more_to_load {
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

    async fn get_account_authorized_depositors(
        &self,
        network_id: NetworkID,
        account_address: AccountAddress,
    ) -> Result<Vec<AccountAuthorizedDepositorsResponseItem>> {
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let mut items: Vec<AccountAuthorizedDepositorsResponseItem> =
            Vec::new();
        let mut more_to_load = true;
        let mut cursor: Option<String> = None;
        while more_to_load {
            let request = AccountAuthorizedDepositorsRequest::new(
                account_address,
                cursor.clone(),
                GATEWAY_PAGE_REQUEST_LIMIT,
            );
            let response = gateway_client
                .account_authorized_depositors(request)
                .await?;
            items.extend(response.items);
            cursor = response.next_cursor;
            more_to_load = cursor.is_some();
        }

        Ok(items)
    }
}
