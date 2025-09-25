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

#[async_trait::async_trait]
pub trait OsAnalyseTxPreview {
    async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        are_instructions_originating_from_host: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview>;
}

#[async_trait::async_trait]
impl OsAnalyseTxPreview for SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Extracting the transaction signers.
    /// 2. Executing the transaction preview GW request.
    /// 3. Running the execution summary with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    async fn analyse_transaction_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        are_instructions_originating_from_host: bool,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<TransactionToReview> {
        let network_id = self.current_network_id()?;
        let transaction_manifest =
            TransactionManifest::new(instructions, network_id, blobs)?;

        transaction_manifest
            .validate_instructions(are_instructions_originating_from_host)?;

        // Get the execution summary
        let execution_summary = self
            .get_execution_summary(
                transaction_manifest.clone(),
                nonce,
                notary_public_key,
                are_instructions_originating_from_host,
            )
            .await?;

        Ok(TransactionToReview {
            transaction_manifest,
            execution_summary,
        })
    }
}

#[async_trait::async_trait]
pub trait OsExecutionSummary {
    async fn get_execution_summary<T: PreviewableManifest + Send + Sync>(
        &self,
        manifest: T,
        nonce: Nonce,
        notary_public_key: PublicKey,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary>;

    fn extract_signer_public_keys(
        &self,
        manifest_summary: &ManifestSummary,
    ) -> Result<IndexSet<PublicKey>>;

    fn extract_proofs(
        &self,
        manifest_summary: &ManifestSummary,
    ) -> Result<IndexMap<AddressOfAccountOrPersona, AccessControllerAddress>>;

    fn extract_execution_summary(
        &self,
        manifest: &dyn DynamicallyAnalyzableManifest,
        receipts: PreviewResponseReceipts,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary>;

    fn map_failed_transaction_preview(
        receipt: TransactionReceipt,
    ) -> CommonError;
}

#[async_trait::async_trait]
impl OsExecutionSummary for SargonOS {
    async fn get_execution_summary<T: PreviewableManifest + Send + Sync>(
        &self,
        manifest: T,
        nonce: Nonce,
        notary_public_key: PublicKey,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary> {
        let summary = manifest.summary()?;

        let signer_public_keys = self.extract_signer_public_keys(&summary)?;
        let proofs = self.extract_proofs(&summary)?;

        let gateway_client = self.gateway_client_with(manifest.network_id());

        let epoch = gateway_client.current_epoch().await?;

        let receipts = manifest
            .fetch_preview(
                proofs,
                &gateway_client,
                epoch,
                signer_public_keys,
                notary_public_key,
                nonce,
            )
            .await?;

        self.extract_execution_summary(
            &manifest,
            receipts,
            are_instructions_originating_from_host,
        )
    }

    fn extract_signer_public_keys(
        &self,
        manifest_summary: &ManifestSummary,
    ) -> Result<IndexSet<PublicKey>> {
        // Extracting the entities requiring auth to check if the notary is signatory
        let profile = self.profile()?;
        let signable_summary =
            SignableManifestSummary::new(manifest_summary.clone());

        // Extracting the signers public keys
        Ok(ExtractorOfInstancesRequiredToSignTransactions::extract(
            &profile,
            vec![signable_summary],
            SigningPurpose::sign_transaction_primary(),
        )?
        .iter()
        .map(|i| i.public_key.public_key)
        .collect::<IndexSet<PublicKey>>())
    }

    fn extract_proofs(
        &self,
        manifest_summary: &ManifestSummary,
    ) -> Result<IndexMap<AddressOfAccountOrPersona, AccessControllerAddress>>
    {
        let entities = manifest_summary
            .addresses_of_accounts_requiring_auth
            .iter()
            .map(|a| self.entity_by_address((*a).into()))
            .chain(
                manifest_summary
                    .addresses_of_personas_requiring_auth
                    .iter()
                    .map(|i| self.entity_by_address((*i).into())),
            )
            .collect::<Result<Vec<_>>>()?;

        let mut proofs = IndexMap::<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >::new();

        entities.iter().for_each(|e| {
            if let Some(control) = e.security_state().as_securified() {
                proofs.insert(e.address(), control.access_controller_address);
            }
        });

        Ok(proofs)
    }

    fn extract_execution_summary(
        &self,
        manifest: &dyn DynamicallyAnalyzableManifest,
        receipts: PreviewResponseReceipts,
        are_instructions_originating_from_host: bool,
    ) -> Result<ExecutionSummary> {
        let receipt = receipts
            .receipt
            .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;

        if receipt.status != TransactionReceiptStatus::Succeeded {
            return Err(Self::map_failed_transaction_preview(receipt));
        };

        let engine_toolkit_receipt = receipts
            .engine_toolkit_receipt
            .ok_or(CommonError::FailedToExtractTransactionReceiptBytes)?;

        let mut execution_summary =
            manifest.execution_summary(engine_toolkit_receipt)?;

        let reserved_manifest_class = execution_summary
            .detailed_classification
            .iter()
            .find(|classification| classification.is_reserved());

        if let Some(reserved_manifest_class) = reserved_manifest_class
            && !are_instructions_originating_from_host
        {
            return Err(CommonError::ReservedManifestClass {
                class: reserved_manifest_class.kind().to_string(),
            });
        }

        execution_summary.classify_securify_entity_if_present(
            |entity_address| {
                self.entity_by_address(entity_address)
                    .and_then(|entity| {
                        entity
                            .get_provisional()
                            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)
                            .and_then(|p| {
                                p.into_factor_instances_derived()
                                    .or(Err(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived))
                            })
                    })
                    .and_then(|security_structure| {
                        self.security_structure_of_factor_sources_from_security_structure_id(
                            security_structure.security_structure_id
                        )
                    })
            },
        )?;

        Ok(execution_summary)
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

#[async_trait::async_trait]
pub trait PreviewableManifest:
    DynamicallyAnalyzableManifest + Send + Sync
{
    async fn fetch_preview(
        &self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        gateway_client: &GatewayClient,
        start_epoch_inclusive: Epoch,
        signer_public_keys: IndexSet<PublicKey>,
        notary_public_key: PublicKey,
        nonce: Nonce,
    ) -> Result<PreviewResponseReceipts>;
}

#[async_trait::async_trait]
impl PreviewableManifest for TransactionManifestV2 {
    async fn fetch_preview(
        &self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        gateway_client: &GatewayClient,
        start_epoch_inclusive: Epoch,
        signer_public_keys: IndexSet<PublicKey>,
        notary_public_key: PublicKey,
        nonce: Nonce,
    ) -> Result<PreviewResponseReceipts> {
        let modified_with_proofs =
            self.modify_add_proofs(entities_with_access_controllers)?;

        let request = TransactionPreviewRequestV2::new_transaction_analysis(
            modified_with_proofs,
            start_epoch_inclusive,
            signer_public_keys,
            notary_public_key,
            nonce,
        )?;

        let response = gateway_client.transaction_preview_v2(request).await?;

        Ok(PreviewResponseReceipts {
            receipt: response.receipt,
            engine_toolkit_receipt: response.radix_engine_toolkit_receipt,
        })
    }
}

#[async_trait::async_trait]
impl PreviewableManifest for TransactionManifest {
    async fn fetch_preview(
        &self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        gateway_client: &GatewayClient,
        start_epoch_inclusive: Epoch,
        signer_public_keys: IndexSet<PublicKey>,
        notary_public_key: PublicKey,
        nonce: Nonce,
    ) -> Result<PreviewResponseReceipts> {
        let modified_with_proofs =
            self.modify_add_proofs(entities_with_access_controllers)?;

        let request = TransactionPreviewRequest::new_transaction_analysis(
            modified_with_proofs,
            start_epoch_inclusive,
            signer_public_keys,
            Some(notary_public_key),
            nonce,
        );
        let response = gateway_client.transaction_preview(request).await?;

        Ok(PreviewResponseReceipts {
            receipt: Some(response.receipt),
            engine_toolkit_receipt: response.radix_engine_toolkit_receipt,
        })
    }
}

pub struct PreviewResponseReceipts {
    receipt: Option<TransactionReceipt>,
    engine_toolkit_receipt:
        Option<ScryptoSerializableToolkitTransactionReceipt>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::*;
    use radix_engine_toolkit_common::receipt::{
        FeeSummary as RETFeeSummary, LockedFees as RETLockedFees,
        StateUpdatesSummary as RETStateUpdatesSummary,
    };
    use sargon_os_factors::prelude::*;

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
        let responses = prepare_preview_response(
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
        let mut first_call_responses = prepare_preview_response(
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
        let mut second_call_responses = prepare_preview_response(
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
        let responses = prepare_preview_response(
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
    async fn execution_summary_parse_error() {
        let responses = prepare_preview_response(
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
                underlying: "NotACommitSuccessReceipt".to_string()
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
                reserved_instructions:
                    "AccountLockFeeAccountUpdateOwnerKeysMetadataField"
                        .to_string()
            })
        )
    }

    use prelude::{fixture_rtm, fixture_tx};
    use radix_common::math::Decimal as ScryptoDecimal192;

    #[actix_rt::test]
    async fn success() {
        let responses = prepare_preview_response(
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

        pretty_assertions::assert_eq!(
            result,
            Ok(TransactionToReview {
                transaction_manifest: manifest,
                execution_summary: ExecutionSummary::new(
                    [],
                    [],
                    [acc],
                    [],
                    [],
                    [ReservedInstruction::AccountLockFee, ReservedInstruction::AccountUpdateOwnerKeysMetadataField],
                    [],
                    [],
                    None,
                    FeeLocks::default(),
                    FeeSummary::new(0, 0, 0, 0,),
                    NewEntities::default()
                )
            })
        )
    }

    #[actix_rt::test]
    async fn signer_entities_not_found() {
        let responses = prepare_preview_response(
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

        // Just asserts that the execution path reached GW preview call
        assert!(matches!(
            result,
            Err(CommonError::ExecutionSummaryFail { .. })
        ))
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
                Nonce::sample(),
                PublicKey::sample(),
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
        let responses = prepare_preview_response_v2(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            TransactionPreviewResponseV2 {
                at_ledger_state_version: 0,
                receipt: Some(TransactionReceipt {
                    status: TransactionReceiptStatus::Succeeded,
                    error_message: None,
                }),
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
                logs: None,
            }
        );
        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;

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
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            result,
            PreAuthToReview::Enclosed(PreAuthEnclosedManifest {
                manifest: subintent_manifest,
                summary: ExecutionSummary::new(
                    [],
                    [],
                    [],
                    [],
                    [],
                    [],
                    [],
                    [],
                    Some(DetailedManifestClass::General),
                    FeeLocks::default(),
                    FeeSummary::new(0, 0, 0, 0,),
                    NewEntities::default()
                )
            })
        )
    }

    #[actix_rt::test]
    async fn test_classify_manifest_as_securify_entity_for_account() {
        let transaction_preview_response =
            fixture_and_json::<TransactionPreviewResponse>(fixture_tx!(
                "apply_security_shield_to_unsecurified_account_execution"
            ))
            .unwrap()
            .0;

        let responses = prepare_preview_response(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            transaction_preview_response,
        );

        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();
        let structure_source_ids_sample =
            SecurityStructureOfFactorSourceIDs::sample();
        os.add_security_structure_of_factor_source_ids(
            &structure_source_ids_sample,
        )
        .await
        .unwrap();

        let profile = os.profile().unwrap();
        let accounts = profile.clone().accounts_on_current_network().unwrap();
        let entity_address_to_securify = accounts
            .first()
            .map(|a| AddressOfAccountOrPersona::from(a.address))
            .unwrap();
        let _ = os
            .apply_security_shield_with_id_to_entities(
                structure_source_ids_sample.id(),
                IndexSet::just(entity_address_to_securify),
            )
            .await
            .unwrap();

        let transaction_to_review = os
            .analyse_transaction_preview(
                fixture_rtm!("apply_security_shield_to_unsecurified_account")
                    .to_owned(),
                Blobs::default(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await
            .unwrap();

        assert_eq!(
            transaction_to_review
                .execution_summary
                .detailed_classification
                .unwrap(),
            DetailedManifestClass::SecurifyEntity {
                entity_address: entity_address_to_securify,
                provisional_security_structure_metadata:
                    structure_source_ids_sample.metadata
            }
        );
    }

    #[actix_rt::test]
    async fn test_classify_manifest_as_securify_entity_for_persona() {
        let transaction_preview_response =
            fixture_and_json::<TransactionPreviewResponse>(fixture_tx!(
                "apply_security_shield_to_unsecurified_persona_execution"
            ))
            .unwrap()
            .0;

        let responses = prepare_preview_response(
            LedgerState {
                network: "".to_string(),
                state_version: 0,
                proposer_round_timestamp: "".to_string(),
                epoch: 0,
                round: 0,
            },
            transaction_preview_response,
        );

        let os =
            prepare_os(MockNetworkingDriver::new_with_bodies(200, responses))
                .await;
        os.with_timeout(|x| x.debug_add_all_sample_hd_factor_sources())
            .await
            .unwrap();
        let structure_source_ids_sample =
            SecurityStructureOfFactorSourceIDs::sample();
        os.add_security_structure_of_factor_source_ids(
            &structure_source_ids_sample,
        )
        .await
        .unwrap();

        let profile = os.profile().unwrap();
        let personas = profile.clone().personas_on_current_network().unwrap();
        let entity_address_to_securify = personas
            .first()
            .map(|p| AddressOfAccountOrPersona::from(p.address))
            .unwrap();
        let _ = os
            .apply_security_shield_with_id_to_entities(
                structure_source_ids_sample.id(),
                IndexSet::just(entity_address_to_securify),
            )
            .await
            .unwrap();

        let transaction_to_review = os
            .analyse_transaction_preview(
                fixture_rtm!("apply_security_shield_to_unsecurified_persona")
                    .to_owned(),
                Blobs::default(),
                true,
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await
            .unwrap();

        assert_eq!(
            transaction_to_review
                .execution_summary
                .detailed_classification
                .unwrap(),
            DetailedManifestClass::SecurifyEntity {
                entity_address: entity_address_to_securify,
                provisional_security_structure_metadata:
                    structure_source_ids_sample.metadata
            }
        );
    }

    fn prepare_manifest_with_account_entity() -> TransactionManifest {
        let account = Account::sample_mainnet();
        TransactionManifest::set_owner_keys_hashes(
            &account.address.into(),
            vec![PublicKeyHash::sample()],
        )
        .modify_add_lock_fee(LockFeeData::new_with_unsecurified_fee_payer(
            account.address(),
            Decimal192::zero(),
        ))
        .unwrap()
    }
}
