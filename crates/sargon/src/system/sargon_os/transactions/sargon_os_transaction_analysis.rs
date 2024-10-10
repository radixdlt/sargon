use std::sync::RwLockWriteGuard;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        message: Message,
        is_wallet_transaction: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        self.perform_transaction_preview_analysis(
            instructions,
            blobs,
            message,
            is_wallet_transaction,
            nonce,
            notary_public_key,
        )
        .await
    }
}

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

    async fn get_transaction_preview(
        &self,
        gateway_client: GatewayClient,
        manifest: TransactionManifest,
        network_id: NetworkID,
        message: Message,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionPreviewResponse> {
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
        if response.receipt.status != TransactionReceiptStatus::Succeeded {
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

    async fn extract_transaction_signers(
        &self,
        manifest_summary: ManifestSummary,
    ) -> Result<IndexSet<PublicKey>> {
        let signer_entities = self
            .extract_transaction_signer_entities(manifest_summary.clone())
            .await?;
        Ok(self.extract_transaction_signers_public_keys(signer_entities))
    }

    async fn extract_transaction_signer_entities(
        &self,
        manifest_summary: ManifestSummary,
    ) -> Result<IndexSet<AccountOrPersona>> {
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
        Ok(signer_entities)
    }

    fn extract_transaction_signers_public_keys(
        &self,
        signers: IndexSet<AccountOrPersona>,
    ) -> IndexSet<PublicKey> {
        signers
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
            .collect()
    }
}

#[cfg(test)]
mod transaction_preview_analysis_tests {
    use super::*;
    use std::sync::Mutex;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn manifest_parse_error() {
        let os = SUT::fast_boot().await;

        let result = os
            .perform_transaction_preview_analysis(
                "instructions".to_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert!(matches!(
            result,
            Err(CommonError::InvalidInstructionsString { .. })
        ));
    }

    #[actix_rt::test]
    async fn profile_not_loaded_error() {
        let os = SUT::fast_boot().await;
        os.profile_state_holder
            .replace_profile_state_with(ProfileState::None)
            .unwrap();

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert!(matches!(
            result,
            Err(CommonError::ProfileStateNotLoaded { .. })
        ));
    }

    #[actix_rt::test]
    async fn failed_network_response_error() {
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(result, Err(CommonError::NetworkResponseBadCode))
    }

    #[actix_rt::test]
    async fn failed_preview_response_unknown_error() {
        let responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: None,
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Failed,
                    error_message: None,
                },
            },
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::FailedTransactionPreview {
                error_message: "Unknown reason".to_string()
            })
        )
    }

    #[actix_rt::test]
    async fn failed_preview_response_deposit_rules_error() {
        let mut responses: Vec<BagOfBytes> = vec![];
        let mut first_call_responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: None,
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Failed,
                    error_message: Some(
                        "AccountError(DepositIsDisallowed".to_string(),
                    ),
                },
            },
        );
        let mut second_call_responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: None,
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Failed,
                    error_message: Some(
                        "AccountError(NotAllBucketsCouldBeDeposited"
                            .to_string(),
                    ),
                },
            },
        );
        responses.append(&mut first_call_responses);
        responses.append(&mut second_call_responses);
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::OneOfReceivingAccountsDoesNotAllowDeposits)
        );

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::OneOfReceivingAccountsDoesNotAllowDeposits)
        )
    }

    #[actix_rt::test]
    async fn missing_radix_engine_toolkit_receipt_error() {
        let responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: None,
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Succeeded,
                    error_message: None,
                },
            },
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;
        let manifest = TransactionManifest::set_owner_keys_hashes(
            &IdentityAddress::sample().into(),
            AccountOrPersona::sample_mainnet_other()
                .virtual_hierarchical_deterministic_factor_instances()
                .into_iter()
                .map(|i| PublicKeyHash::hash(i.public_key.public_key))
                .collect(),
        );

        let result = os
            .perform_transaction_preview_analysis(
                manifest.instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::FailedToExtractTransactionReceiptBytes)
        )
    }

    #[actix_rt::test]
    async fn execution_summary_parse_error() {
        let responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: Some(
                    ScryptoSerializableToolkitTransactionReceipt::Reject {
                        reason: "Test".to_string(),
                    },
                ),
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Succeeded,
                    error_message: None,
                },
            },
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;

        let result = os
            .perform_transaction_preview_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::ExecutionSummaryFail {
                underlying: "InvalidReceipt".to_string()
            })
        )
    }

    #[actix_rt::test]
    async fn execution_summary_reserved_instructions_error() {
        let responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: Some(ScryptoSerializableToolkitTransactionReceipt::CommitSuccess {
                    state_updates_summary: native_radix_engine_toolkit::receipt::StateUpdatesSummary {
                        new_entities: IndexSet::new(),
                        metadata_updates: IndexMap::new(),
                        non_fungible_data_updates: IndexMap::new(),
                        newly_minted_non_fungibles: IndexSet::new(),
                    },
                    worktop_changes: IndexMap::new(),
                    fee_summary: native_radix_engine_toolkit::receipt::FeeSummary {
                        execution_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        finalization_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        storage_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        royalty_fees_in_xrd: ScryptoDecimal192::zero().into(),
                    },
                    locked_fees: native_radix_engine_toolkit::receipt::LockedFees {
                        contingent: ScryptoDecimal192::zero().into(),
                        non_contingent: ScryptoDecimal192::zero().into(),
                    },
                }),
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Succeeded,
                    error_message: None,
                },
            },
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;

        let result = os
            .perform_transaction_preview_analysis(
                Instructions::sample_mainnet().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                false,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: "AccountLockFee".to_string()
            })
        )
    }

    #[actix_rt::test]
    async fn success() {
        let responses = prepare_responses(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponse {
                encoded_receipt: "".to_string(),
                radix_engine_toolkit_receipt: Some(ScryptoSerializableToolkitTransactionReceipt::CommitSuccess {
                    state_updates_summary: native_radix_engine_toolkit::receipt::StateUpdatesSummary {
                        new_entities: IndexSet::new(),
                        metadata_updates: IndexMap::new(),
                        non_fungible_data_updates: IndexMap::new(),
                        newly_minted_non_fungibles: IndexSet::new(),
                    },
                    worktop_changes: IndexMap::new(),
                    fee_summary: native_radix_engine_toolkit::receipt::FeeSummary {
                        execution_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        finalization_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        storage_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        royalty_fees_in_xrd: ScryptoDecimal192::zero().into(),
                    },
                    locked_fees: native_radix_engine_toolkit::receipt::LockedFees {
                        contingent: ScryptoDecimal192::zero().into(),
                        non_contingent: ScryptoDecimal192::zero().into(),
                    },
                }),
                logs: vec![],
                receipt: TransactionReceipt {
                    status: TransactionReceiptStatus::Succeeded,
                    error_message: None,
                },
            },
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;
        let acc: AccountAddress = "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".into();

        let result = os
            .analyse_transaction_preview(
                Instructions::sample_mainnet().instructions_string(),
                Blobs::default(),
                Message::sample(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Ok(
                TransactionToReview {
                    transaction_manifest: TransactionManifest::sample(),
                    execution_summary: ExecutionSummary::new(
                        [
                            (
                                acc,
                                vec![
                                    ResourceIndicator::fungible(
                                        "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
                                        FungibleResourceIndicator::guaranteed("1337")
                                    ),
                                ]
                            )
                        ], //withdrawals
                        [
                            (
                                AccountAddress::from("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"),
                                vec![
                                    ResourceIndicator::fungible(
                                        "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
                                        FungibleResourceIndicator::guaranteed("1337")
                                    ),
                                ]
                            ),
                        ], //deposits
                        [acc],
                        [],
                        [],
                        [ReservedInstruction::AccountLockFee],
                        [],
                        [],
                        [
                            DetailedManifestClass::Transfer {
                                is_one_to_one: true
                            },
                            DetailedManifestClass::General
                        ],
                        FeeLocks::default(),
                        FeeSummary::new(
                            "0",
                            "0",
                            "0",
                            0,
                        ),
                        NewEntities::default()
            )
                }))
    }

    async fn prepare_os(
        mock_networking_driver: MockNetworkingDriver,
    ) -> Arc<SargonOS> {
        let req = SUT::boot_test_with_networking_driver(Arc::new(
            mock_networking_driver,
        ));
        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();
        os.profile_state_holder
            .update_profile_with(|profile| {
                Ok(profile.networks.insert(ProfileNetwork::sample_mainnet()))
            })
            .unwrap();
        os
    }

    fn prepare_responses(
        ledger_state: LedgerState,
        preview_response: TransactionPreviewResponse,
    ) -> Vec<BagOfBytes> {
        vec![
            to_bag_of_bytes(TransactionConstructionResponse { ledger_state }),
            to_bag_of_bytes(preview_response),
        ]
    }

    fn to_bag_of_bytes<T>(value: T) -> BagOfBytes
    where
        T: Serialize,
    {
        BagOfBytes::from(serde_json::to_vec(&value).unwrap())
    }
}
