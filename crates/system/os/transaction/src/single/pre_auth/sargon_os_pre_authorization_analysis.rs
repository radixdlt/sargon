use crate::prelude::*;

/// Host-controlled options for [`OsAnalysePreAuthPreview::analyse_pre_auth_preview_with_options`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PreAuthPreviewOptions {
    /// Allow `create_proof` on AccessControllers that do not secure any entity
    /// in this wallet's Profile, i.e. multisig accounts controlled by an
    /// external AccessController. Off by default. See
    /// [`ExternalAccessControllerProofsPolicy`] for the exact conditions.
    pub allow_external_access_controller_proofs: bool,
}

#[async_trait::async_trait]
pub trait OsAnalysePreAuthPreview {
    async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<PreAuthToReview>;

    async fn analyse_pre_auth_preview_with_options(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
        options: PreAuthPreviewOptions,
    ) -> Result<PreAuthToReview>;
}

#[async_trait::async_trait]
impl OsAnalysePreAuthPreview for SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Creating the SubintentManifest.
    /// 2. Validating if the manifest is open or enclosed.
    /// 3. If open, the manifest with its summary is returned.
    /// 4. If enclosed, it extracts the transaction signers and then transaction preview GW request is executed.
    /// 3. The execution summary is created with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<PreAuthToReview> {
        self.analyse_pre_auth_preview_with_options(
            instructions,
            blobs,
            nonce,
            notary_public_key,
            PreAuthPreviewOptions::default(),
        )
        .await
    }

    /// Same as [`OsAnalysePreAuthPreview::analyse_pre_auth_preview`], with
    /// host-controlled [`PreAuthPreviewOptions`].
    async fn analyse_pre_auth_preview_with_options(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
        options: PreAuthPreviewOptions,
    ) -> Result<PreAuthToReview> {
        debug!(
            "Pre-auth review: received subintent manifest (instructions_len={}, blobs={})",
            instructions.len(),
            blobs.0.len()
        );
        let network_id = self.current_network_id();
        debug!("Pre-auth review: using network_id={:?}", network_id);
        let subintent_manifest = SubintentManifest::new(
            instructions,
            network_id,
            blobs.clone(),
            ChildSubintentSpecifiers::default(),
        )?;
        debug!("Pre-auth review: subintent manifest constructed");

        // PreAuth transaction cannot be sent by the Host itself, so reserved
        // instructions are always checked. The host may opt in to the narrow
        // external-AccessController exemption.
        let external_access_controller_proofs =
            if options.allow_external_access_controller_proofs {
                Some(self.external_access_controller_proofs_policy()?)
            } else {
                None
            };
        let summary = subintent_manifest
            .validated_summary_for_pre_authorization(
                external_access_controller_proofs.as_ref(),
            )?;
        debug!("Pre-auth review: subintent manifest summary validated");

        let pre_auth_to_review = match subintent_manifest.as_enclosed() {
            Some(manifest) => {
                debug!("Pre-auth review: enclosed subintent manifest detected");
                let execution_summary = self
                    .get_execution_summary(
                        manifest,
                        nonce,
                        notary_public_key,
                        false,
                        true,
                    )
                    .await?;
                debug!("Pre-auth review: enclosed preview execution summary computed");

                PreAuthToReview::Enclosed(PreAuthEnclosedManifest {
                    manifest: subintent_manifest,
                    summary: execution_summary,
                })
            }
            None => {
                debug!("Pre-auth review: open subintent manifest detected");
                PreAuthToReview::Open(PreAuthOpenManifest {
                    manifest: subintent_manifest,
                    summary,
                })
            }
        };

        debug!("Pre-auth review: completed");
        Ok(pre_auth_to_review)
    }
}

trait ExternalAccessControllerProofsPolicyBuilding {
    fn external_access_controller_proofs_policy(
        &self,
    ) -> Result<ExternalAccessControllerProofsPolicy>;
}

impl ExternalAccessControllerProofsPolicyBuilding for SargonOS {
    /// AccessControllers that secure any account or persona in the Profile, on
    /// any network, including hidden and tombstoned entities. These are never
    /// exempt.
    fn external_access_controller_proofs_policy(
        &self,
    ) -> Result<ExternalAccessControllerProofsPolicy> {
        let profile = self.profile()?;
        let access_controllers = profile
            .networks
            .iter()
            .flat_map(|network| {
                network
                    .accounts
                    .iter()
                    .map(AccountOrPersona::from)
                    .chain(network.personas.iter().map(AccountOrPersona::from))
                    .collect_vec()
            })
            .filter_map(|entity| {
                entity
                    .security_state()
                    .as_securified()
                    .map(|control| control.access_controller_address)
            })
            .collect_vec();
        Ok(ExternalAccessControllerProofsPolicy::new(
            access_controllers,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::*;
    use radix_common::math::Decimal as ScryptoDecimal192;
    use radix_engine_toolkit_common::receipt::{
        FeeSummary as RETFeeSummary, LockedFees as RETLockedFees,
        StateUpdatesSummary as RETStateUpdatesSummary,
    };
    use sargon_os_factors::prelude::*;

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

    // ==================
    // External AccessController proofs
    // ==================

    fn multisig_spend_instructions(ac: AccessControllerAddress) -> String {
        format!(
            r#"
CALL_METHOD Address("{ac}") "create_proof";
CALL_METHOD Address("{account}") "withdraw" Address("{xrd}") Decimal("10");
CALL_METHOD Address("{recipient}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
YIELD_TO_PARENT;
"#,
            account = AccountAddress::sample_mainnet(),
            xrd = ResourceAddress::sample_mainnet_xrd(),
            recipient = AccountAddress::sample_mainnet_other(),
        )
    }

    fn allowing_external_access_controller_proofs() -> PreAuthPreviewOptions {
        PreAuthPreviewOptions {
            allow_external_access_controller_proofs: true,
        }
    }

    #[test]
    fn external_access_controller_proofs_are_off_by_default() {
        assert!(
            !PreAuthPreviewOptions::default()
                .allow_external_access_controller_proofs
        );
    }

    #[actix_rt::test]
    async fn create_proof_on_external_access_controller_is_rejected_by_default()
    {
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;
        let instructions = multisig_spend_instructions(
            AccessControllerAddress::sample_mainnet(),
        );

        let result = os
            .analyse_pre_auth_preview(
                instructions.clone(),
                Blobs::default(),
                Nonce::sample(),
                PublicKey::sample(),
            )
            .await;
        assert_eq!(
            result,
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: "AccessControllerMethod".to_owned()
            })
        );

        let with_default_options = os
            .analyse_pre_auth_preview_with_options(
                instructions,
                Blobs::default(),
                Nonce::sample(),
                PublicKey::sample(),
                PreAuthPreviewOptions::default(),
            )
            .await;
        assert_eq!(with_default_options, result);
    }

    #[actix_rt::test]
    async fn create_proof_on_external_access_controller_is_permitted_when_enabled(
    ) {
        let os = prepare_os(MockNetworkingDriver::new_always_failing()).await;
        let instructions = multisig_spend_instructions(
            AccessControllerAddress::sample_mainnet(),
        );

        let result = os
            .analyse_pre_auth_preview_with_options(
                instructions.clone(),
                Blobs::default(),
                Nonce::sample(),
                PublicKey::sample(),
                allowing_external_access_controller_proofs(),
            )
            .await
            .unwrap();

        let subintent_manifest = SubintentManifest::new(
            instructions,
            NetworkID::Mainnet,
            Blobs::default(),
            ChildSubintentSpecifiers::default(),
        )
        .unwrap();
        let summary = subintent_manifest.summary().unwrap();
        // The host can still see that an AccessController's authority is used.
        assert_eq!(
            summary.reserved_instructions,
            vec![ReservedInstruction::AccessControllerMethod]
        );
        assert_eq!(
            result,
            PreAuthToReview::Open(PreAuthOpenManifest {
                manifest: subintent_manifest,
                summary,
            })
        );
    }

    #[actix_rt::test]
    async fn create_proof_on_access_controller_of_profile_account_is_rejected_when_enabled(
    ) {
        let securified = Account::sample_all()
            .into_iter()
            .find(|a| a.security_state().as_securified().is_some())
            .expect("a securified account sample");
        let own_access_controller = securified
            .security_state()
            .as_securified()
            .unwrap()
            .access_controller_address;
        let os = prepare_os_with_entities([securified], []).await;

        let result = os
            .analyse_pre_auth_preview_with_options(
                multisig_spend_instructions(own_access_controller),
                Blobs::default(),
                Nonce::sample(),
                PublicKey::sample(),
                allowing_external_access_controller_proofs(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: "AccessControllerMethod".to_owned()
            })
        );
    }

    #[actix_rt::test]
    async fn create_proof_on_access_controller_of_profile_persona_is_rejected_when_enabled(
    ) {
        let securified = Persona::sample_all()
            .into_iter()
            .find(|p| p.security_state().as_securified().is_some())
            .expect("a securified persona sample");
        let own_access_controller = securified
            .security_state()
            .as_securified()
            .unwrap()
            .access_controller_address;
        let os = prepare_os_with_entities([], [securified]).await;

        let result = os
            .analyse_pre_auth_preview_with_options(
                multisig_spend_instructions(own_access_controller),
                Blobs::default(),
                Nonce::sample(),
                PublicKey::sample(),
                allowing_external_access_controller_proofs(),
            )
            .await;

        assert_eq!(
            result,
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions: "AccessControllerMethod".to_owned()
            })
        );
    }
}
