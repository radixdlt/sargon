use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsTransactionIntentBuilder: Send + Sync {
    async fn build_transaction_intents(
        &self,
        network_id: NetworkID,
        manifests_with_entities_with_xrd_balance: Vec<
            SecurityShieldApplication,
        >,
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

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
        let end_epoch_exclusive = Epoch::one_week_from(start_epoch_inclusive);

        let mut transaction_id_to_notary_private_key: IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        > = IndexMap::new();

        let mut build_intent = |manifest: &TransactionManifest,
                                nonce: Nonce,
                                notary_private_key_bytes: Exactly32Bytes|
         -> Result<TransactionIntent> {
            let notary_private_key = Ed25519PrivateKey::from_exactly32_bytes(
                notary_private_key_bytes,
            );
            let notary_public_key = notary_private_key.public_key();
            let header = TransactionHeader::new(
                network_id,
                start_epoch_inclusive,
                end_epoch_exclusive,
                nonce,
                notary_public_key,
                NotaryIsSignatory(true),
                0,
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

        let with_intents = manifests_with_entities_with_xrd_balance.into_iter().map(|m| {
            // We tactically use the same nonce for all variants of the TransactionIntents
            // for securified entities - ensuring that we cannot accidentally submit
            // two variants of the same application.
            let nonce =  Nonce::random();
            // We can use the same notary private key for all variants since they
            // are in fact the same application
            let notary_private_key_bytes = Exactly32Bytes::generate();
            match m {
                SecurityShieldApplication::ForUnsecurifiedEntity(unsec) => {
                    let intent = build_intent(unsec.manifest(), nonce, notary_private_key_bytes)?;
                    let with_intents = SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent::with_intent(unsec, intent);

                    Ok(SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(with_intents))
                }
                SecurityShieldApplication::ForSecurifiedEntity(sec) => {
                    let with_intents: SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents = {

                        let initiate_with_recovery_complete_with_primary = build_intent(sec.initiate_with_recovery_complete_with_primary(), nonce, notary_private_key_bytes)?;

                        let initiate_with_recovery_complete_with_confirmation = build_intent(sec.initiate_with_recovery_complete_with_confirmation(), nonce, notary_private_key_bytes)?;

                        let initiate_with_recovery_delayed_completion = build_intent(sec.initiate_with_recovery_delayed_completion(), nonce, notary_private_key_bytes)?;

                        let initiate_with_primary_complete_with_confirmation = build_intent(sec.initiate_with_primary_complete_with_confirmation(), nonce, notary_private_key_bytes)?;

                        let initiate_with_primary_delayed_completion = build_intent(sec.initiate_with_primary_delayed_completion(), nonce, notary_private_key_bytes)?;

                        Ok(SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents::with_intents(sec, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
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
