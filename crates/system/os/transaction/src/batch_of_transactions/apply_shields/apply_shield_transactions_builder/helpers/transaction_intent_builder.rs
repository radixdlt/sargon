use crate::prelude::*;

/// A transaction intent builder for applying shields, fetches current
/// epoch, generates nonces and ephemeral notary keys, and builds
/// TransactionIntent for each `SecurityShieldApplication`(which contains
/// many TransactionManifest variants for manifests applying shield to
/// securified entities).
#[async_trait::async_trait]
pub trait ApplyShieldTransactionsTransactionIntentBuilder: Send + Sync {
    /// Fetches current epoch, generates nonces and ephemeral notary keys,
    /// and builds TransactionIntent for each `SecurityShieldApplication`
    /// (which contains many TransactionManifest variants for manifests
    /// applying shield to securified entities).
    ///
    /// Returns a `ApplySecurityShieldPayloadToSign` which is ready to be signed using
    /// the signing manager.
    async fn build_transaction_intents(
        &self,
        network_id: NetworkID,
        manifests_with_entities_with_xrd_balance: Vec<
            SecurityShieldApplication,
        >,
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

/// A transaction intent builder for applying shields, fetches current
/// epoch, generates nonces and ephemeral notary keys, and builds
/// TransactionIntent for each `SecurityShieldApplication`(which contains
/// many TransactionManifest variants for manifests applying shield to
/// securified entities).
pub struct ApplyShieldTransactionsTransactionIntentBuilderImpl {
    networking_driver: Arc<dyn NetworkingDriver>,
}
impl ApplyShieldTransactionsTransactionIntentBuilderImpl {
    pub fn new(networking_driver: Arc<dyn NetworkingDriver>) -> Self {
        Self { networking_driver }
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsTransactionIntentBuilder
    for ApplyShieldTransactionsTransactionIntentBuilderImpl
{
    /// Fetches current epoch, generates nonces and ephemeral notary keys,
    /// and builds TransactionIntent for each `SecurityShieldApplication`
    /// (which contains many TransactionManifest variants for manifests
    /// applying shield to securified entities).
    ///
    /// Returns a `ApplySecurityShieldPayloadToSign` which is ready to be signed using
    /// the signing manager.
    async fn build_transaction_intents(
        &self,
        network_id: NetworkID,
        manifests_with_entities_with_xrd_balance: Vec<
            SecurityShieldApplication,
        >,
    ) -> Result<ApplySecurityShieldPayloadToSign> {
        let gateway_client =
            GatewayClient::new(self.networking_driver.clone(), network_id);

        let start_epoch_inclusive = gateway_client.current_epoch().await?;

        // We give the user/host (Radix wallet app) one month to submit
        // all the transactions
        let end_epoch_exclusive =
            Epoch::max_window_from_start(start_epoch_inclusive);

        let mut transaction_id_to_notary_private_key: IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        > = IndexMap::new();

        // Builds a TransactionIntent for a given manifest, intent_discriminator and notary private key.
        // by forming a header with the fetched epoch.
        let mut build_intent = |manifest: &TransactionManifest,
                                fee_tip_percentage: Option<u16>,
                                notary_private_key_bytes: Exactly32Bytes|
         -> Result<TransactionIntent> {
            // We want and MUST use different IntentDiscriminators, since it is
            // otherwise possible for two manifest variants to have the same
            // TransactionIntentHash (TXID) - at least for weak shields, specifically
            // InitiateWithRecoveryCompleteWithPrimary and
            // InitiateWithRecoveryCompleteWithConfirmation.
            // we must ensure those manifest will have different TXID.
            let intent_discriminator = IntentDisciminator32::random();

            let notary_private_key = Ed25519PrivateKey::from_exactly32_bytes(
                notary_private_key_bytes,
            );
            let notary_public_key = notary_private_key.public_key();
            let header = TransactionHeader::new(
                network_id,
                start_epoch_inclusive,
                end_epoch_exclusive,
                intent_discriminator,
                notary_public_key,
                NotaryIsSignatory(false),
                fee_tip_percentage.unwrap_or(0),
            );

            let intent = TransactionIntent::new(
                header,
                manifest.clone(),
                Message::None,
            )?;

            // So we can return the notary keys to callee so we can notarize later.
            //
            // For securified entities the same notary private key will be present under many TransactionIntentHash
            // map keys (identifier).
            transaction_id_to_notary_private_key
                .insert(intent.transaction_intent_hash(), notary_private_key);

            Ok(intent)
        };

        // Map each `SecurityShieldApplication` to a `SecurityShieldApplicationWithTransactionIntents`
        let with_intents = manifests_with_entities_with_xrd_balance.into_iter().map(|shield_application| {
            // We can use the same notary private key for all variants since they
            // are in fact the same application
            let notary_private_key_bytes = Exactly32Bytes::generate();
            match shield_application {

                SecurityShieldApplication::ForUnsecurifiedEntity(unsec) => {
                    let intent = build_intent(
                        unsec.manifest(),
                        unsec.fee_tip_percentage(),
                        notary_private_key_bytes,
                    )?;
                    let with_intent = SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent::with_intent(unsec, intent);

                    Ok(SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(with_intent))
                }
                SecurityShieldApplication::ForSecurifiedEntity(sec) => {
                    let with_intents: SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents = {

                        let fee_tip_percentage = sec.fee_tip_percentage();
                        let initiate_with_recovery_complete_with_primary = build_intent(
                            sec.initiate_with_recovery_complete_with_primary(),
                            fee_tip_percentage,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_recovery_complete_with_confirmation = build_intent(
                            sec.initiate_with_recovery_complete_with_confirmation(),
                            fee_tip_percentage,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_recovery_delayed_completion = build_intent(
                            sec.initiate_with_recovery_delayed_completion(),
                            fee_tip_percentage,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_primary_complete_with_confirmation = build_intent(
                            sec.initiate_with_primary_complete_with_confirmation(),
                            fee_tip_percentage,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_primary_delayed_completion = build_intent(
                            sec.initiate_with_primary_delayed_completion(),
                            fee_tip_percentage,
                            notary_private_key_bytes
                        )?;

                        Ok(
                            SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents::with_intents(
                                sec,
                                initiate_with_recovery_complete_with_primary,
                                initiate_with_recovery_complete_with_confirmation,
                                initiate_with_recovery_delayed_completion,
                                initiate_with_primary_complete_with_confirmation,
                                initiate_with_primary_delayed_completion
                            )
                        )
                    }?;
                    Ok(SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(with_intents))
                }
            }
        }).collect::<Result<Vec<SecurityShieldApplicationWithTransactionIntents>>>()?;

        let payload_to_sign = ApplySecurityShieldPayloadToSign {
            applications_with_intents: with_intents,
            notary_keys: transaction_id_to_notary_private_key,
        };

        Ok(payload_to_sign)
    }
}
