use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsBuilder: Send + Sync {
    async fn build_payload_to_sign(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

pub struct ApplyShieldTransactionsBuilderImpl {}
impl ApplyShieldTransactionsBuilderImpl {
    pub fn new<'a>(os: &'a SargonOS) -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsBuilder for ApplyShieldTransactionsBuilderImpl {
    /// Builds transaction intents of manifests
    /// which applies a shields to entities.
    async fn build_payload_to_sign(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<ApplySecurityShieldPayloadToSign> {
        /*
        // Map Address -> Entity by Profile lookup
        let manifests_with_entities_without_xrd_balances =
            self.lookup_entities_for_manifests(manifest_and_payer_tuples)?;

        // Assert that payer if specified is not part of the batch of entities applying shield
        self.assert_that_payer_is_not_in_batch_of_entities_applying_shield(
            &manifests_with_entities_without_xrd_balances,
        )?;

        // Get XRD balances of XRD Vaults of Access Controllers and Accounts
        let manifests_with_entities_with_xrd_balance = self
            .get_xrd_balances(
                network_id,
                manifests_with_entities_without_xrd_balances,
            )
            .await?;

        // For Securified entities => create 4 other variants of the manifest
        // (noop for Unsecurified entities)
        let applications_without_intents = self
            .create_many_manifest_variants_per_roles_combination(
                manifests_with_entities_with_xrd_balance,
            )?;

        // Build TransactionIntents for all of these applications (5 intents for securified entities)
        // all using the same Epoch window (one week).
        let payload_to_sign = self
            .build_transaction_intents(network_id, applications_without_intents)
            .await?;

        // Persist notary private keys to be able to cancel transactions
        self.persist_notary_private_keys_to_be_able_to_cancel_transactions(
            &payload_to_sign.notary_keys,
        )
        .await?;

        Ok(payload_to_sign)
        */
        todo!()
    }
}
