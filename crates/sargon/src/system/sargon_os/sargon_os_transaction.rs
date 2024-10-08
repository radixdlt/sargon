use std::sync::RwLockWriteGuard;

use crate::prelude::*;

impl SargonOS {
    pub async fn perform_transaction_preview_analysis(
        &self,
        instructions: String,
        blobs: Blobs,
        message: Message,
        is_wallet_transaction: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        let network_id = self.profile_state_holder.current_network_id()?;
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );
        let transaction_manifest =
            TransactionManifest::new(instructions, network_id, blobs)?;

        // Get the transaction preview
        let transaction_preview = self
            .get_transaction_preview(
                gateway_client,
                transaction_manifest.clone(),
                network_id,
                message,
                nonce,
                notary_public_key,
            )
            .await?;
        let engine_toolkit_receipt = transaction_preview
            .radix_engine_toolkit_receipt
            .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;

        // Analyze the manifest
        let execution_summary = transaction_manifest
            .execution_summary_with_engine_toolkit_receipt(
                engine_toolkit_receipt,
            )?;

        // Transactions created outside of the Wallet are not allowed to use reserved instructions
        if !is_wallet_transaction
            && !execution_summary.reserved_instructions.is_empty()
        {
            return Err(
                CommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: execution_summary
                        .reserved_instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect(),
                },
            );
        }

        Ok(TransactionToReview {
            transaction_manifest,
            execution_summary,
        })
    }

    async fn extract_transaction_signers(
        &self,
        manifest_summary: ManifestSummary,
    ) -> Result<IndexSet<PublicKey>> {
        let accounts =
            self.profile_state_holder.accounts_on_current_network()?;
        let personas =
            self.profile_state_holder.personas_on_current_network()?;

        let account_entities: IndexSet<_> = manifest_summary
            .addresses_of_accounts_requiring_auth
            .iter()
            .filter_map(|address| {
                accounts
                    .iter()
                    .find(|account| account.address == *address)
                    .map(AccountOrPersona::AccountEntity)
            })
            .collect();
        let persona_entities: IndexSet<_> = manifest_summary
            .addresses_of_personas_requiring_auth
            .iter()
            .filter_map(|address| {
                personas
                    .iter()
                    .find(|persona| persona.address == *address)
                    .map(AccountOrPersona::PersonaEntity)
            })
            .collect();
        let mut signer_entities: IndexSet<AccountOrPersona> = IndexSet::new();
        signer_entities.extend(account_entities);
        signer_entities.extend(persona_entities);
        Ok(signer_entities
            .iter()
            .flat_map(|entity| {
                let public_keys: IndexSet<PublicKey> = entity
                    .virtual_hierarchical_deterministic_factor_instances()
                    .iter()
                    .map(|factor_instance| {
                        factor_instance.public_key.public_key
                    })
                    .collect();
                public_keys
            })
            .collect())
    }

    async fn get_transaction_preview(
        &self,
        gateway_client: GatewayClient,
        manifest: TransactionManifest,
        network_id: NetworkID,
        message: Message,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionPreviewResponse> {
        // Get all transaction signers
        let signer_public_keys =
            self.extract_transaction_signers(manifest.summary()).await?;
        let epoch = gateway_client.current_epoch().await?;
        let header = TransactionHeader::new(
            network_id,
            epoch,
            Epoch::from(epoch.0 + 10),
            nonce,
            notary_public_key,
            signer_public_keys.is_empty(),
            0,
        );
        let intent = TransactionIntent::new(header, manifest, message)?;
        let request = TransactionPreviewRequest::new(
            intent,
            signer_public_keys,
            TransactionPreviewRequestFlags::new(true, false, false),
        );
        let response = gateway_client.transaction_preview(request).await?;
        if response.receipt.status != TransactionStatus::Succeeded {
            return Err(Self::map_failed_transaction_preview(response));
        };
        Ok(response)
    }

    fn map_failed_transaction_preview(
        response: TransactionPreviewResponse,
    ) -> CommonError {
        let message = response
            .receipt
            .error_message
            .unwrap_or_else(|| "Unknown reason".to_string());

        // Quite rudimentary, but it is not worth making something smarter,
        // as the GW will provide in the future strongly typed errors
        let is_failure_due_to_deposit_rules = message
            .contains("AccountError(DepositIsDisallowed")
            || message.contains("AccountError(NotAllBucketsCouldBeDeposited");

        if is_failure_due_to_deposit_rules {
            CommonError::OneOfReceivingAccountsDoesNotAllowDeposits
        } else {
            CommonError::FailedTransactionPreview {
                error_message: message,
            }
        }
    }
}
