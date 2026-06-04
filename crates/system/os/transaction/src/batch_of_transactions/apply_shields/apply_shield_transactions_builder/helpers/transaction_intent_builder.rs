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
    gateways: SavedGateways,
}
impl ApplyShieldTransactionsTransactionIntentBuilderImpl {
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        gateways: SavedGateways,
    ) -> Self {
        Self {
            networking_driver,
            gateways,
        }
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
        let gateway_client = super::gateway_client_for_network(
            self.networking_driver.clone(),
            &self.gateways,
            network_id,
        );

        let start_epoch_inclusive = gateway_client.current_epoch().await?;

        // We give the user/host (Radix wallet app) one month to submit
        // all the transactions
        let end_epoch_exclusive =
            Epoch::max_window_from_start(start_epoch_inclusive);

        let mut transaction_id_to_notary_private_key: IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        > = IndexMap::new();

        // Builds a TransactionIntent for a given manifest, nonce and notary private key.
        // by forming a header with the fetched epoch.
        let mut build_intent = |manifest: &TransactionManifest,
                                fee_tip_percentage: Option<u16>,
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
            // We tactically use the same nonce for all variants of the TransactionIntents
            // for securified entities - ensuring that we cannot accidentally submit
            // two variants of the same application.
            let nonce =  Nonce::random();
            // We can use the same notary private key for all variants since they
            // are in fact the same application
            let notary_private_key_bytes = Exactly32Bytes::generate();
            match shield_application {
                SecurityShieldApplication::ForUnsecurifiedEntity(unsec) => {
                    let intent = build_intent(
                        unsec.manifest(),
                        unsec.fee_tip_percentage(),
                        nonce,
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
                            nonce,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_recovery_complete_with_confirmation = build_intent(
                            sec.initiate_with_recovery_complete_with_confirmation(),
                            fee_tip_percentage,
                            nonce,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_recovery_delayed_completion = build_intent(
                            sec.initiate_with_recovery_delayed_completion(),
                            fee_tip_percentage,
                            nonce,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_primary_complete_with_confirmation = build_intent(
                            sec.initiate_with_primary_complete_with_confirmation(),
                            fee_tip_percentage,
                            nonce,
                            notary_private_key_bytes
                        )?;

                        let initiate_with_primary_delayed_completion = build_intent(
                            sec.initiate_with_primary_delayed_completion(),
                            fee_tip_percentage,
                            nonce,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ApplyShieldTransactionsTransactionIntentBuilderImpl;

    #[actix_rt::test]
    async fn build_transaction_intents_uses_custom_gateway_url_for_epoch() {
        let response = TransactionConstructionResponse::new(LedgerState::new(
            NetworkID::Mainnet.logical_name(),
            1,
            "2021-01-01T00:00:00Z",
            1,
            1,
        ));
        let body = serde_json::to_vec(&response).unwrap();
        let networking_driver = Arc::new(MockNetworkingDriver::with_spy(
            200,
            body,
            |request, _| {
                assert_eq!(
                    request.url.host_str(),
                    Some("custom-mainnet.example")
                );
                assert_eq!(request.url.path(), "/transaction/construction");
            },
        ));
        let custom_gateway = Gateway::new(
            "https://custom-mainnet.example/".to_string(),
            NetworkID::Mainnet,
        )
        .unwrap();
        let sut =
            SUT::new(networking_driver, SavedGateways::new(custom_gateway));

        let result = sut
            .build_transaction_intents(NetworkID::Mainnet, vec![])
            .await
            .unwrap();

        assert!(result.applications_with_intents.is_empty());
        assert!(result.notary_keys.is_empty());
    }
}
