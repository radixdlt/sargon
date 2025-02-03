use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsBuilder: Send + Sync {
    async fn build_payload_to_sign(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

pub struct ApplyShieldTransactionsBuilderImpl {
    profile_lens: Arc<dyn ApplyShieldTransactionsProfileLens>,
    xrd_balances_fetcher: Arc<dyn ApplyShieldTransactionsXrdBalancesFetcher>,
    poly_manifest_builder: Arc<dyn ApplyShieldTransactionsPolyManifestBuilder>,
    transaction_intent_builder:
        Arc<dyn ApplyShieldTransactionsTransactionIntentBuilder>,
}

impl ApplyShieldTransactionsBuilderImpl {
    pub fn new(os: &SargonOS) -> Result<Self> {
        os.profile().map(|profile| {
            let networking_driver = os.http_client.driver.clone();
            Self {
                profile_lens: Arc::new(
                    ApplyShieldTransactionsProfileLensImpl::new(profile),
                ),
                xrd_balances_fetcher: Arc::new(
                    ApplyShieldTransactionsXrdBalancesFetcherImpl::new(
                        networking_driver.clone(),
                    ),
                ),
                poly_manifest_builder: Arc::new(
                    ApplyShieldTransactionsPolyManifestBuilderImpl::new(),
                ),
                transaction_intent_builder: Arc::new(
                    ApplyShieldTransactionsTransactionIntentBuilderImpl::new(
                        networking_driver,
                    ),
                ),
            }
        })
    }

    async fn persist_notary_private_keys_to_be_able_to_cancel_transactions(
        &self,
        _transaction_id_to_notary_private_key: &IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        >,
    ) -> Result<()> {
        // We do not support this yet, but might in the future.
        info!("Skipped persisting notary private keys to be able to cancel transactions");
        Ok(())
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
        // Map Address -> Entity by Profile lookup
        let manifests_with_entities_without_xrd_balances = self
            .profile_lens
            .lookup_entities_for_manifests(manifest_and_payer_tuples)?;

        // Get XRD balances of XRD Vaults of Access Controllers and Accounts
        let manifests_with_entities_with_xrd_balance = self
            .xrd_balances_fetcher
            .get_xrd_balances(
                network_id,
                manifests_with_entities_without_xrd_balances,
            )
            .await?;

        // For Securified entities => create 4 other variants of the manifest
        // (noop for Unsecurified entities)
        let applications_without_intents = self
            .poly_manifest_builder
            .create_many_manifest_variants_per_roles_combination(
                manifests_with_entities_with_xrd_balance,
            )?;

        // Build TransactionIntents for all of these applications (5 intents for securified entities)
        // all using the same Epoch window (one week).
        let payload_to_sign = self
            .transaction_intent_builder
            .build_transaction_intents(network_id, applications_without_intents)
            .await?;

        // Persist notary private keys to be able to cancel transactions
        self.persist_notary_private_keys_to_be_able_to_cancel_transactions(
            &payload_to_sign.notary_keys,
        )
        .await?;

        Ok(payload_to_sign)
    }
}
