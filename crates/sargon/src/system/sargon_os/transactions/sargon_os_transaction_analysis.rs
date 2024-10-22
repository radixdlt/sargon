use crate::prelude::*;
use radix_transactions::prelude::InstructionV2;
use std::sync::RwLockWriteGuard;

#[uniffi::export]
impl SargonOS {
    /// Performs initial transaction analysis for a given raw manifest v2, including:
    /// 1. Extracting the transaction signers.
    /// 2. Checking the version of the transaction.
    /// 3. Executing the transaction preview GW request for enclosed manifest and running the
    /// execution summary with the manifest and receipt for transaction v1.
    /// 4. Checking if the manifest is enclosed or open for transaction v2.
    /// For enclosed manifest:
    ///     - Executing the transaction preview GW request for enclosed manifest.
    ///     - Running the execution summary with the manifest and receipt.
    /// For open manifest:
    ///     - Extracting the manifest summary.
    /// Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        message: Message,
        version: TransactionVersion,
        are_instructions_originating_from_host: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        self.perform_transaction_analysis(
            instructions,
            blobs,
            message,
            version,
            are_instructions_originating_from_host,
            nonce,
            notary_public_key,
        )
        .await
    }
}

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
    pub async fn perform_transaction_analysis(
        &self,
        instructions: impl AsRef<str>,
        blobs: Blobs,
        message: Message,
        version: TransactionVersion,
        are_instructions_originating_from_host: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        let network_id = self.profile_state_holder.current_network_id()?;

        match version {
            TransactionVersion::V1 => {
                self.handle_transaction_v1(
                    instructions,
                    blobs,
                    message,
                    network_id,
                    nonce,
                    notary_public_key,
                    are_instructions_originating_from_host,
                )
                .await
            }
            TransactionVersion::V2 => {
                self.handle_transaction_v2(
                    instructions,
                    blobs,
                    message,
                    network_id,
                    nonce,
                    notary_public_key,
                    are_instructions_originating_from_host,
                )
                .await
            }
        }
    }

    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Extracting the transaction signers.
    /// 2. Analyzing statically the manifest.
    /// 3. Executing the transaction preview GW request
    /// 4. Running the execution summary with the manifest and receipt.
    /// 5. Mapping relevant errors to ensure proper handling by the hosts.
    ///
    /// Returns `TransactionToReview::V1`, which includes the manifest and the execution summary.
    async fn handle_transaction_v1(
        &self,
        instructions: impl AsRef<str>,
        blobs: Blobs,
        message: Message,
        network_id: NetworkID,
        nonce: Nonce,
        notary_public_key: PublicKey,
        are_instructions_originating_from_host: bool,
    ) -> Result<TransactionToReview> {
        let transaction_manifest =
            TransactionManifest::new(instructions, network_id, blobs)?;

        // Static analysis
        let manifest_summary = transaction_manifest.clone().summary()?;

        // Get the transaction preview receipt
        let engine_toolkit_receipt = self
            .request_transaction_receipt(
                transaction_manifest.clone(),
                manifest_summary,
                network_id,
                message,
                nonce,
                notary_public_key,
            )
            .await?;

        // Dynamic analysis
        let execution_summary =
            transaction_manifest.execution_summary(engine_toolkit_receipt)?;

        self.check_reserved_instructions(
            are_instructions_originating_from_host,
            execution_summary.clone(),
        )?;

        Ok(TransactionToReview::V1 {
            transaction_manifest,
            execution_summary,
        })
    }

    /// Performs initial transaction analysis for a given raw manifest v2, including:
    /// 1. Extracting the transaction signers.
    /// 2. Analyzing statically the manifest.
    /// 3. Checking if the manifest is enclosed or open.
    /// For enclosed manifest:
    ///     - Executing the transaction preview GW request for enclosed manifest.
    ///     - Running the execution summary with the manifest and receipt.
    /// For open manifest:
    ///     - Extracting the manifest summary.
    /// 4. Mapping relevant errors to ensure proper handling by the hosts.
    ///
    /// Returns `TransactionToReview::V2`.
    async fn handle_transaction_v2(
        &self,
        instructions: impl AsRef<str>,
        blobs: Blobs,
        message: Message,
        network_id: NetworkID,
        nonce: Nonce,
        notary_public_key: PublicKey,
        are_instructions_originating_from_host: bool,
    ) -> Result<TransactionToReview> {
        let transaction_manifest = TransactionManifestV2::new(
            instructions,
            network_id,
            blobs.clone(),
            // For now, pre-authorization has no need to support children for any use cases we're targeting
            ChildIntents::empty(),
        )?;

        // Static analysis
        let manifest_summary = transaction_manifest.clone().summary()?;

        let summary = if transaction_manifest.is_enclosed() {
            // In order for the GW preview request to work as expected,
            // we need to filter out the instructions marking the manifest as enclosed
            // TODO when RET support is added

            // Get the transaction preview receipt
            let engine_toolkit_receipt = self
                .request_transaction_receipt(
                    TransactionManifest::sample(),
                    manifest_summary,
                    network_id,
                    message,
                    nonce,
                    notary_public_key,
                )
                .await?;

            // Analyze the manifest
            let execution_summary = transaction_manifest
                .execution_summary(engine_toolkit_receipt)?;

            self.check_reserved_instructions(
                are_instructions_originating_from_host,
                execution_summary.clone(),
            )?;

            TransactionToReviewV2Summary::Enclosed { execution_summary }
        } else {
            TransactionToReviewV2Summary::Open { manifest_summary }
        };

        Ok(TransactionToReview::V2 {
            transaction_manifest,
            summary,
        })
    }

    /// Checks if the transaction contains reserved instructions if originating from host.
    /// Transactions created outside of the host are not allowed to use reserved instructions.
    fn check_reserved_instructions(
        &self,
        are_instructions_originating_from_host: bool,
        execution_summary: ExecutionSummary,
    ) -> Result<()> {
        if !are_instructions_originating_from_host
            && !execution_summary.reserved_instructions.is_empty()
        {
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: execution_summary
                    .reserved_instructions
                    .iter()
                    .map(|i| i.to_string())
                    .collect(),
            })
        } else {
            Ok(())
        }
    }

    async fn request_transaction_receipt(
        &self,
        manifest: TransactionManifest, //TODO remove manifest dependency
        manifest_summary: ManifestSummary,
        network_id: NetworkID,
        message: Message,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<ScryptoSerializableToolkitTransactionReceipt> {
        let gateway_client = GatewayClient::new(
            self.clients.http_client.driver.clone(),
            network_id,
        );

        // Getting the current ledger epoch
        let epoch = gateway_client.current_epoch().await?;

        // Extracting the entities requiring auth to check if the notary is signatory
        let profile = self.profile_state_holder.profile()?;
        let entities_requiring_auth =
            ExtractorOfEntitiesRequiringAuth::extract(
                &profile,
                manifest_summary,
            )?;

        // Creating the transaction header and intent
        let header = TransactionHeader::new(
            network_id,
            epoch,
            Epoch::window_end_from_start(epoch),
            nonce,
            notary_public_key,
            entities_requiring_auth.is_empty(),
            0,
        );
        let intent = TransactionIntent::new(header, manifest, message)?;

        // Extracting the signers public keys
        let signer_public_keys =
            ExtractorOfInstancesRequiredToSignTransactions::extract(
                &profile,
                vec![intent.clone()],
                RoleKind::Primary,
            )?
            .iter()
            .map(|i| i.public_key.public_key)
            .collect::<IndexSet<PublicKey>>();

        // Making the transaction preview Gateway request
        let request = TransactionPreviewRequest::new(
            intent,
            signer_public_keys,
            TransactionPreviewRequestFlags::default(),
        );
        let response = gateway_client.transaction_preview(request).await?;

        // Checking the transaction receipt status and mapping the response
        if response.receipt.status != TransactionReceiptStatus::Succeeded {
            return Err(Self::map_failed_transaction_preview(response));
        };

        // Extracting the Radix Engine Toolkit receipt
        response
            .radix_engine_toolkit_receipt
            .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)
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
mod transaction_analysis_tests {
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
        let test = |version: TransactionVersion| async {
            let os = SUT::fast_boot().await;

            let result = os
                .perform_transaction_analysis(
                    "instructions".to_string(),
                    Blobs::sample(),
                    Message::sample(),
                    version,
                    false,
                    Nonce::sample(),
                    PublicKey::sample(),
                )
                .await;

            assert!(matches!(
                result,
                Err(CommonError::InvalidInstructionsString { .. })
            ));
        };

        test(TransactionVersion::V1).await;
        test(TransactionVersion::V2).await;
    }

    #[actix_rt::test]
    async fn profile_not_loaded_error() {
        let test = |version: TransactionVersion| async {
            let os = SUT::fast_boot().await;
            os.profile_state_holder
                .replace_profile_state_with(ProfileState::None)
                .unwrap();

            let result = os
                .perform_transaction_analysis(
                    TransactionManifest::sample().instructions_string(),
                    Blobs::sample(),
                    Message::sample(),
                    TransactionVersion::V1,
                    false,
                    Nonce::sample(),
                    PublicKey::sample(),
                )
                .await;

            assert!(matches!(
                result,
                Err(CommonError::ProfileStateNotLoaded { .. })
            ));
        };

        test(TransactionVersion::V1).await;
        test(TransactionVersion::V2).await;
    }

    #[actix_rt::test]
    async fn failed_network_response_error() {
        let test = |version: TransactionVersion| async {
            let os =
                prepare_os(MockNetworkingDriver::new_always_failing()).await;

            let result = os
                .perform_transaction_analysis(
                    prepare_manifest_with_account_entity()
                        .instructions_string(),
                    Blobs::sample(),
                    Message::sample(),
                    TransactionVersion::V1,
                    false,
                    Nonce::sample(),
                    PublicKey::sample(),
                )
                .await;

            assert_eq!(result, Err(CommonError::NetworkResponseBadCode))
        };

        test(TransactionVersion::V1).await;
        test(TransactionVersion::V2).await;
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
            .perform_transaction_analysis(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
        let manifest = prepare_manifest_with_account_entity();

        let result = os
            .perform_transaction_analysis(
                manifest.instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
            .perform_transaction_analysis(
                manifest.instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
            vec![PublicKeyHash::sample()],
        );

        let result = os
            .perform_transaction_analysis(
                manifest.instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
            .perform_transaction_analysis(
                TransactionManifest::sample().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
                false,
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
            .perform_transaction_analysis(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
        let ret_zero: AsStr<Decimal> = Decimal::ZERO.into();
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
                        execution_fees_in_xrd: ret_zero,
                        finalization_fees_in_xrd: ret_zero,
                        storage_fees_in_xrd: ret_zero,
                        royalty_fees_in_xrd: ret_zero,
                    },
                    locked_fees: RETLockedFees {
                        contingent: ret_zero,
                        non_contingent: ret_zero,
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
            .perform_transaction_analysis(
                prepare_manifest_with_account_entity().instructions_string(),
                Blobs::sample(),
                Message::sample(),
                TransactionVersion::V1,
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
                Message::sample(),
                TransactionVersion::V1,
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;

        assert_eq!(
            result,
            Ok(TransactionToReview::V1 {
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
