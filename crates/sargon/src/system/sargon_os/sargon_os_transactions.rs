use crate::prelude::*;

// ==================
// Submit Transaction
// ==================
#[uniffi::export]
impl SargonOS {
    /// Submits a notarized transaction payload to the network.
    pub async fn submit_transaction(
        &self,
        notarized_transaction: NotarizedTransaction,
    ) -> Result<IntentHash> {
        let network_id = self.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        gateway_client
            .submit_notarized_transaction(notarized_transaction)
            .await
    }

    /// Submits a compiled transaction payload to the network.
    pub async fn submit_compiled_transaction(
        &self,
        compiled_notarized_intent: CompiledNotarizedIntent,
    ) -> Result<IntentHash> {
        self.submit_transaction(compiled_notarized_intent.decompile())
            .await
    }
}
