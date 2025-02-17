use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsAnalysePreAuthPreview {
    async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
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
        let network_id = self.current_network_id()?;
        let subintent_manifest = SubintentManifest::new(
            instructions,
            network_id,
            blobs.clone(),
            ChildSubintentSpecifiers::default(),
        )?;

        let summary = subintent_manifest.validated_summary(
            network_id,
            false, // PreAuth transaction cannot be sent by the Host itself
        )?;

        let pre_auth_to_review = match subintent_manifest.as_enclosed_scrypto()
        {
            Some(manifest) => {
                let execution_summary = self
                    .get_execution_summary(
                        network_id,
                        manifest,
                        nonce,
                        notary_public_key,
                        false,
                    )
                    .await?;

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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

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
}
