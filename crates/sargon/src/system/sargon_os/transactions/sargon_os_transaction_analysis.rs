use std::sync::RwLockWriteGuard;

use radix_engine_toolkit::functions::transaction_v2::{subintent_manifest, transaction_manifest};
use radix_transactions::{manifest::BlobProvider, model::TransactionPayload};

use crate::prelude::*;

/// This is part of an error message returned **by Gateway**, indicating the deposits are denied for the account.
/// We use it part of logic below, matching against this String - we really should upgrade this code to be more
/// structured - we MUST update this value if Gateway where to change this value.
const GW_ERR_ACCOUNT_DEPOSIT_DISALLOWED: &str =
    "AccountError(DepositIsDisallowed";
/// This is part of an error message returned **by Gateway**, indicating the deposits are denied for the account.
/// We use it part of logic below, matching against this String - we really should upgrade this code to be more
/// structured - we MUST update this value if Gateway where to change this value.
const GW_ERR_NOT_ALL_COULD_BE_DEPOSITED: &str =
    "AccountError(NotAllBucketsCouldBeDeposited";

impl SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Extracting the transaction signers.
    /// 2. Executing the transaction preview GW request.
    /// 3. Running the execution summary with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        are_instructions_originating_from_host: bool,
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

        let summary = transaction_manifest.summary()?;

        // Transactions created outside of the Wallet are not allowed to use reserved instructions
        if !are_instructions_originating_from_host
            && !summary.reserved_instructions.is_empty()
        {
            return Err(
                CommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: summary
                        .reserved_instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect(),
                },
            );
        }

        // Get the execution summary
        let execution_summary = self
            .get_transaction_execution_summary(
                gateway_client,
                transaction_manifest.clone(),
                nonce,
                Some(notary_public_key),
                are_instructions_originating_from_host,
            )
            .await?;

        Ok(TransactionToReview {
            transaction_manifest,
            execution_summary,
        })
    }

    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Creating the SubintentManifest.
    /// 2. Validating if the manifest is open or enclosed.
    /// 3. If open, the manifest with its summary is returned.
    /// 4. If enclosed, it extracts the transaction signers and then transaction preview GW request is executed.
    /// 3. The execution summary is created with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        intent_discrimnator: IntentDiscriminator
    ) -> Result<PreAuthToReview> {
        let network_id = self.profile_state_holder.current_network_id()?;
        let subintent_manifest = SubintentManifest::new(
            instructions,
            network_id,
            blobs.clone(),
            ChildSubintentSpecifiers::default(),
        )?;

        let summary = subintent_manifest.summary()?;

        if !summary.reserved_instructions.is_empty() {
            return Err(
                CommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: summary
                        .reserved_instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect(),
                },
            );
        }

        let pre_auth_to_review = match subintent_manifest.as_enclosed_scrypto() {
            Some(manifest) => {
                let execution_summary = self.get_transaction_execution_summary_v2(
                    network_id, 
                    manifest, 
                    intent_discrimnator, 
                    false // PreAuth transaction cannot be sent by the Host itself
                ).await?;

                PreAuthToReview::Enclosed(PreAuthEnclosedManifest {
                    manifest: subintent_manifest,
                    summary: execution_summary,
                })
            }
            None => PreAuthToReview::Open(PreAuthOpenManifest {
                manifest: subintent_manifest,
                summary,
            }),
        };

        Ok(pre_auth_to_review)
    }
}

impl SargonOS {
    fn extract_signer_public_keys(
        &self,
        manifest_summary: ManifestSummary,
    ) -> Result<IndexSet<PublicKey>> {
        // Extracting the entities requiring auth to check if the notary is signatory
        let profile = self.profile_state_holder.profile()?;
        let signable_summary =
            SignableManifestSummary::new(manifest_summary.clone());

        // Extracting the signers public keys
        Ok(ExtractorOfInstancesRequiredToSignTransactions::extract(
            &profile,
            vec![signable_summary],
            RoleKind::Primary,
        )?
        .iter()
        .map(|i| i.public_key.public_key)
        .collect::<IndexSet<PublicKey>>())
    }

    async fn get_transaction_execution_summary_v2(
        &self,
        network_id: NetworkID,
        manifest: ScryptoTransactionManifestV2,
        discriminator: IntentDiscriminator,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary> {
        let summary = RET_statically_analyze_and_validate_v2(&manifest).map_err(map_static_analysis_error)?;

        let signer_public_keys =
        self.extract_signer_public_keys(
            ManifestSummary::from((summary, network_id.clone()))
        )?;

        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        let epoch = gateway_client.current_epoch().await?;

        let request = TransactionPreviewRequestV2::new_transaction_analysis(
            manifest.clone(), 
            epoch, 
            signer_public_keys,
             PublicKey::sample(), 
             discriminator, 
             network_id
        );
        

        let response = gateway_client.transaction_preview_v2(request).await?;


        let receipt = response
        .receipt
        .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;
    
            if receipt.status != TransactionReceiptStatus::Succeeded {
                return Err(Self::map_failed_transaction_preview(receipt));
            };

        
        let engine_toolkit_receipt = response
        .radix_engine_toolkit_receipt
        .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;

        let execution_summary = manifest.execution_summary(engine_toolkit_receipt, network_id)?;

    let reserved_manifest_class = execution_summary
        .detailed_classification
        .iter()
        .find(|classification| classification.is_reserved());

    if let Some(reserved_manifest_class) = reserved_manifest_class
        && !are_instructions_originating_from_host
    {
        return Err(CommonError::ReservedManifestClass {
            class: reserved_manifest_class.clone(),
        });
    }

    Ok(execution_summary)
    }

    async fn get_transaction_execution_summary(
        &self,
        gateway_client: GatewayClient,
        manifest: TransactionManifest,
        nonce: Nonce,
        notary_public_key: Option<PublicKey>,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary> {
        let signer_public_keys =
            self.extract_signer_public_keys(manifest.summary()?)?;

        // Getting the current ledger epoch
        let epoch = gateway_client.current_epoch().await?;

        let request = TransactionPreviewRequest::new_transaction_analysis_v1(
            manifest.clone(),
            epoch,
            signer_public_keys,
            notary_public_key,
            nonce,
        );
        let response = gateway_client.transaction_preview(request).await?;

        // Checking the transaction receipt status and mapping the response
        if response.receipt.status != TransactionReceiptStatus::Succeeded {
            return Err(Self::map_failed_transaction_preview(response.receipt));
        };

        let engine_toolkit_receipt = response
            .radix_engine_toolkit_receipt
            .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;

        let execution_summary =
            manifest.execution_summary(engine_toolkit_receipt)?;

        let reserved_manifest_class = execution_summary
            .detailed_classification
            .iter()
            .find(|classification| classification.is_reserved());

        if let Some(reserved_manifest_class) = reserved_manifest_class
            && !are_instructions_originating_from_host
        {
            return Err(CommonError::ReservedManifestClass {
                class: reserved_manifest_class.kind(),
            });
        }

        Ok(execution_summary)
    }

    #[cfg(not(tarpaulin_include))] // TBD
    #[allow(dead_code)]
    async fn get_subintent_preview(
        &self,
        gateway_client: GatewayClient,
        manifest: TransactionManifestV2,
        _nonce: Nonce,
    ) -> Result<TransactionPreviewResponse> {
        let _signer_public_keys =
            self.extract_signer_public_keys(manifest.summary()?)?;

        // Getting the current ledger epoch
        let _epoch = gateway_client.current_epoch().await?;

        unimplemented!("To be defined when GW is available, likely that there will be a new endpoint with new payload definition")
    }

    fn map_failed_transaction_preview(
        receipt: TransactionReceipt,
    ) -> CommonError {
        let message = receipt
            .error_message
            .unwrap_or_else(|| "Unknown reason".to_string());

        // Quite rudimentary, but it is not worth making something smarter,
        // as the GW will provide in the future strongly typed errors
        let is_failure_due_to_deposit_rules = message
            .contains(GW_ERR_ACCOUNT_DEPOSIT_DISALLOWED)
            || message.contains(GW_ERR_NOT_ALL_COULD_BE_DEPOSITED);

        if is_failure_due_to_deposit_rules {
            CommonError::OneOfReceivingAccountsDoesNotAllowDeposits
        } else {
            CommonError::FailedTransactionPreview {
                error_message: message,
            }
        }
    }
}

#[cfg(test)]
mod transaction_preview_analysis_tests {
    use super::*;
    use radix_common::prelude::Decimal;
    use radix_engine_toolkit_common::receipt::{
        AsStr, FeeSummary as RETFeeSummary, LockedFees as RETLockedFees,
        StateUpdatesSummary as RETStateUpdatesSummary,
    };
    use std::sync::Mutex;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn manifest_parse_error() {
        let os = SUT::fast_boot().await;

        let result = os
            .analyse_transaction_preview(
                "instructions".to_string(),
                Blobs::sample(),
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
            .analyse_transaction_preview(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
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
            .analyse_transaction_preview(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::NetworkResponseBadCode { code: 500 })
        )
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
            .analyse_transaction_preview(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                true,
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
        let manifest = prepare_manifest_with_account_entity();

        let result = os
            .analyse_transaction_preview(
                manifest.instructions_string(),
                Blobs::sample(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::OneOfReceivingAccountsDoesNotAllowDeposits)
        );

        let result = os
            .analyse_transaction_preview(
                manifest.instructions_string(),
                Blobs::sample(),
                true,
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
            vec![PublicKeyHash::sample()],
        );

        let result = os
            .analyse_transaction_preview(
                manifest.instructions_string(),
                Blobs::sample(),
                true,
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
    async fn signer_entities_not_found() {
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
            .analyse_transaction_preview(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(result, Err(CommonError::UnknownAccount))
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
            .analyse_transaction_preview(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                true,
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
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;

        let result = os
            .analyse_transaction_preview(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
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
                    state_updates_summary: RETStateUpdatesSummary {
                        new_entities: IndexSet::new(),
                        metadata_updates: IndexMap::new(),
                        non_fungible_data_updates: IndexMap::new(),
                        newly_minted_non_fungibles: IndexSet::new(),
                    },
                    worktop_changes: IndexMap::new(),
                    fee_summary: RETFeeSummary {
                        execution_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        finalization_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        storage_fees_in_xrd: ScryptoDecimal192::zero().into(),
                        royalty_fees_in_xrd: ScryptoDecimal192::zero().into(),
                    },
                    locked_fees: RETLockedFees {
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
        let acc: AccountAddress = Account::sample().address;
        let manifest = prepare_manifest_with_account_entity();

        let result = os
            .analyse_transaction_preview(
                manifest.instructions_string(),
                Blobs::default(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Ok(TransactionToReview {
                transaction_manifest: manifest,
                execution_summary: ExecutionSummary::new(
                    [],
                    [],
                    [acc],
                    [],
                    [],
                    [ReservedInstruction::AccountLockFee],
                    [],
                    [],
                    [],
                    FeeLocks::default(),
                    FeeSummary::new("0", "0", "0", 0,),
                    NewEntities::default()
                )
            })
        )
    }

    #[actix_rt::test]
    async fn analyse_open_pre_auth_preview() {
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;

        let scrypto_subintent_manifest =
            ScryptoSubintentManifestV2Builder::new_subintent_v2()
                .assert_worktop_is_empty()
                .drop_all_proofs()
                .yield_to_parent(())
                .yield_to_parent(())
                .build();

        let subintent_manifest: SubintentManifest =
            (scrypto_subintent_manifest, NetworkID::Mainnet)
                .try_into()
                .unwrap();

        let result = os
            .analyse_pre_auth_preview(
                subintent_manifest.manifest_string(),
                Blobs::default(),
                IntentDiscriminator::sample(),
            )
            .await
            .unwrap();

        assert_eq!(
            result,
            PreAuthToReview::Open(PreAuthOpenManifest {
                manifest: subintent_manifest.clone(),
                summary: subintent_manifest.summary().unwrap(),
            })
        )
    }

    #[actix_rt::test]
    async fn analyse_open_enclosed_auth_preview() {
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;

        let scrypto_subintent_manifest =
            ScryptoSubintentManifestV2Builder::new_subintent_v2()
                .assert_worktop_is_empty()
                .drop_all_proofs()
                .yield_to_parent(())
                .build();

        let subintent_manifest: SubintentManifest =
            (scrypto_subintent_manifest, NetworkID::Mainnet)
                .try_into()
                .unwrap();

        let result = os
            .analyse_pre_auth_preview(
                subintent_manifest.manifest_string(),
                Blobs::default(),
                IntentDiscriminator::sample(),
            )
            .await
            .unwrap();

        assert_eq!(
            result,
            PreAuthToReview::Enclosed(PreAuthEnclosedManifest {
                manifest: subintent_manifest.clone(),
                summary: ExecutionSummary::sample_stokenet(),
            })
        )
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
                profile.networks.insert(ProfileNetwork::sample_mainnet());
                profile.factor_sources.insert(FactorSource::sample());
                Ok(())
            })
            .unwrap();
        os
    }

    fn prepare_manifest_with_account_entity() -> TransactionManifest {
        let account = Account::sample_mainnet();
        TransactionManifest::set_owner_keys_hashes(
            &account.address.into(),
            vec![PublicKeyHash::sample()],
        )
        .modify_add_lock_fee(&account.address, Some(Decimal192::zero()))
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
