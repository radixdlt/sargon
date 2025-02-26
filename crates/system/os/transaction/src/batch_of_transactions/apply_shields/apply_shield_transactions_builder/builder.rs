use crate::prelude::*;

/// Builder of `ApplySecurityShieldPayloadToSign`.
#[async_trait::async_trait]
pub trait ApplyShieldTransactionsBuilder: Send + Sync {
    /// Prepares a batch of Applications of SecurityShields ready to be signed
    /// from a list of Manifests and Payers.
    ///
    /// This is a complex multi-step process:
    /// 1. Lookup entities for each manifest and payer
    /// 2. Fetch XRD balances for each entity
    /// 3. Create 5 variants of each manifest for securified entities
    /// 4. Build TransactionIntents for each of these applications
    /// 5. Persist notary private keys to be able to cancel transactions
    async fn build_payload_to_sign(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

/// Builder of `ApplySecurityShieldPayloadToSign`.
pub struct ApplyShieldTransactionsBuilderImpl {
    /// For looking up entities by address
    profile_view: Arc<dyn ApplyShieldTransactionsProfileView>,

    /// For fetching XRD balances of entities (also XRD vault of Accesses Controllers)
    xrd_balances_fetcher: Arc<dyn ApplyShieldTransactionsXrdBalancesFetcher>,

    /// For creating 5 variants of each manifest for securified entities - one manifest
    /// for each role combination. Including addition of lock fee and top up of XRD vault
    /// instructions. One variant for unsecurified entities.
    poly_manifest_builder: Arc<dyn ApplyShieldTransactionsPolyManifestBuilder>,

    /// For building TransactionIntents for each of application of security shield.
    transaction_intent_builder:
        Arc<dyn ApplyShieldTransactionsTransactionIntentBuilder>,
}

impl ApplyShieldTransactionsBuilderImpl {
    /// Creates a new instance of `ApplyShieldTransactionsBuilderImpl`, using
    /// SargonOS and its clients to init helpers.
    pub fn new(os: &SargonOS) -> Result<Self> {
        os.profile()
            .map(|profile| Self::with(profile, os.http_client.driver.clone()))
    }
    pub fn with(
        profile: Profile,
        networking_driver: Arc<dyn NetworkingDriver>,
    ) -> Self {
        Self {
            profile_view: Arc::new(
                ApplyShieldTransactionsProfileViewImpl::new(profile),
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
    }

    /// Persists notary private keys to be able to cancel transactions.
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
    /// Prepares a batch of Applications of SecurityShields ready to be signed
    /// from a list of Manifests and Payers.
    ///
    /// This is a complex multistep process:
    /// 1. Lookup entities for each manifest and payer
    /// 2. Fetch XRD balances for each entity
    /// 3. Create 5 variants of each manifest for securified entities
    /// 4. Build TransactionIntents for each of these applications
    /// 5. Persist notary private keys to be able to cancel transactions
    async fn build_payload_to_sign(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<ApplySecurityShieldPayloadToSign> {
        // Map Address -> Entity by Profile lookup
        let manifests_with_entities_without_xrd_balances = self
            .profile_view
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
