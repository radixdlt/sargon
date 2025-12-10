use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput as ScryptoAccessControllerCancelRecoveryRoleRecoveryProposalManifestInput,
    AccessControllerTimedConfirmRecoveryInput as ScryptoAccessControllerTimedConfirmRecoveryInput,
    ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
    ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
};

pub trait TransactionManifestCancelRecoveryProposal {
    /// Ensures that any pending recovery proposal initiated by the recovery role is
    /// cancelled before the manifest executes other recovery-related instructions.
    /// If a timed recovery is in progress, it will first be stopped.
    ///
    /// The stop and cancellation instructions are prepended only when the Access Controller
    /// state reports an outstanding recovery attempt and the manifest will initiate recovery
    /// using the recovery role (any `InitiateWithRecovery*` combination).
    fn apply_cancel_recovery_proposal_instruction(
        &self,
        ac_state_details: &AccessControllerStateDetails,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Self;
}

impl TransactionManifestCancelRecoveryProposal for TransactionManifest {
    fn apply_cancel_recovery_proposal_instruction(
        &self,
        ac_state_details: &AccessControllerStateDetails,
        role_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Self {
        let recovery_attempt =
            &ac_state_details.state.recovery_role_recovery_attempt;

        let initiates_with_recovery_role = matches!(
            role_combination,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithPrimary
                | RolesExercisableInTransactionManifestCombination::
                    InitiateWithRecoveryCompleteWithConfirmation
                | RolesExercisableInTransactionManifestCombination::
                    InitiateWithRecoveryDelayedCompletion
        );

        if recovery_attempt.is_none() || !initiates_with_recovery_role {
            return self.clone();
        }

        let recovery_attempt = recovery_attempt.as_ref().unwrap();
        let is_timed_recovery =
            recovery_attempt.allow_timed_recovery_after.is_some();

        let mut builder = ScryptoTransactionManifestBuilder::new();

        // If timed recovery is in progress, stop it first
        if is_timed_recovery {
            let factors_and_time =
                AccessControllerFactorsAndTimeInput::with_recovery_proposal(
                    recovery_attempt.recovery_proposal.clone(),
                );
            builder = builder.call_method(
                ac_state_details.address.scrypto(),
                SCRYPTO_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
                ScryptoAccessControllerTimedConfirmRecoveryInput::from(
                    &factors_and_time,
                ),
            );
        }

        // Cancel the recovery proposal
        builder = builder.call_method(
            ac_state_details.address.scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
            ScryptoAccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {},
        );

        builder = builder.extend_builder_with_manifest(self.clone());

        TransactionManifest::sargon_built(builder, self.network_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn empty_manifest() -> TransactionManifest {
        TransactionManifest::empty(NetworkID::Mainnet)
    }

    fn sample_ac_state_details(
        address: AccessControllerAddress,
        include_recovery_attempt: bool,
    ) -> AccessControllerStateDetails {
        sample_ac_state_details_with_timed(
            address,
            include_recovery_attempt,
            false,
        )
    }

    fn sample_ac_state_details_with_timed(
        address: AccessControllerAddress,
        include_recovery_attempt: bool,
        is_timed_recovery: bool,
    ) -> AccessControllerStateDetails {
        AccessControllerStateDetails::new(
            address,
            AccessControllerFieldStateValue {
                controlled_vault: EntityReference {
                    entity_type: CoreApiEntityType::InternalFungibleVault,
                    is_global: false,
                    entity_address: "internal_vault".to_owned(),
                },
                xrd_fee_vault: None,
                timed_recovery_delay_minutes: None,
                recovery_badge_resource_address:
                    ResourceAddress::sample_mainnet(),
                is_primary_role_locked: false,
                primary_role_recovery_attempt: None,
                has_primary_role_badge_withdraw_attempt: false,
                recovery_role_recovery_attempt: include_recovery_attempt
                    .then(|| sample_recovery_attempt(is_timed_recovery)),
                has_recovery_role_badge_withdraw_attempt: false,
            },
            Decimal192::ten(),
        )
    }

    fn sample_recovery_attempt(is_timed: bool) -> RecoveryRoleRecoveryAttempt {
        RecoveryRoleRecoveryAttempt {
            recovery_proposal: RecoveryProposal {
                primary_role: AccessRule::AllowAll,
                recovery_role: AccessRule::AllowAll,
                confirmation_role: AccessRule::AllowAll,
                timed_recovery_delay_minutes: if is_timed {
                    Some(1440)
                } else {
                    None
                },
            },
            allow_timed_recovery_after: if is_timed {
                Some(ScryptoInstantDto {
                    unix_timestamp_seconds: "1730999831".to_string(),
                    date_time: Some("2024-11-07T11:17:11Z".to_string()),
                })
            } else {
                None
            },
        }
    }

    #[test]
    fn prepends_cancel_recovery_when_attempt_exists_and_recovery_role_initiates(
    ) {
        let ac_address = AccessControllerAddress::sample_mainnet();
        let manifest_str = format!(
            r#"
            CALL_METHOD
                Address("{ac}")
                "stop_timed_recovery"
                Tuple()
            ;
            "#,
            ac = ac_address,
        );
        let manifest = TransactionManifest::new(
            &manifest_str,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        let ac_state_details =
            sample_ac_state_details(ac_address.clone(), true);

        let updated_manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(
            updated_manifest,
            format!(
                r#"
                CALL_METHOD
                    Address("{ac}")
                    "cancel_recovery_role_recovery_proposal"
                ;
                CALL_METHOD
                    Address("{ac}")
                    "stop_timed_recovery"
                    Tuple()
                ;
                "#,
                ac = ac_address,
            ),
        );
    }

    #[test]
    fn skips_cancel_when_attempt_missing() {
        let ac_address = AccessControllerAddress::sample_mainnet();
        let manifest_str = format!(
            r#"
            CALL_METHOD
                Address("{ac}")
                "stop_timed_recovery"
                Tuple()
            ;
            "#,
            ac = ac_address,
        );
        let manifest = TransactionManifest::new(
            &manifest_str,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        let ac_state_details =
            sample_ac_state_details(ac_address.clone(), false);

        let updated_manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryCompleteWithConfirmation,
        );

        manifest_eq(updated_manifest, manifest_str);
    }

    #[test]
    fn skips_cancel_when_primary_role_initiates() {
        let ac_address = AccessControllerAddress::sample_mainnet();
        let manifest_str = format!(
            r#"
            CALL_METHOD
                Address("{ac}")
                "stop_timed_recovery"
                Tuple()
            ;
            "#,
            ac = ac_address,
        );
        let manifest = TransactionManifest::new(
            &manifest_str,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        let ac_state_details =
            sample_ac_state_details(ac_address.clone(), true);

        let updated_manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithPrimaryCompleteWithConfirmation,
        );

        manifest_eq(updated_manifest, manifest_str);
    }

    #[test]
    fn prepends_stop_and_cancel_when_timed_recovery_in_progress() {
        let ac_address = AccessControllerAddress::sample_mainnet();
        let manifest_str = format!(
            r#"
            CALL_METHOD
                Address("{ac}")
                "init_recovery"
                Tuple()
            ;
            "#,
            ac = ac_address,
        );
        let manifest = TransactionManifest::new(
            &manifest_str,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        let ac_state_details =
            sample_ac_state_details_with_timed(ac_address.clone(), true, true);

        let updated_manifest = manifest.apply_cancel_recovery_proposal_instruction(
            &ac_state_details,
            RolesExercisableInTransactionManifestCombination::
                InitiateWithRecoveryDelayedCompletion,
        );

        // Should prepend both stop_timed_recovery and cancel instructions
        let manifest_string = updated_manifest.to_string();
        assert!(manifest_string.contains("stop_timed_recovery"));
        assert!(
            manifest_string.contains("cancel_recovery_role_recovery_proposal")
        );

        // Stop should come before cancel
        let stop_pos = manifest_string.find("stop_timed_recovery").unwrap();
        let cancel_pos = manifest_string
            .find("cancel_recovery_role_recovery_proposal")
            .unwrap();
        assert!(stop_pos < cancel_pos, "stop_timed_recovery should come before cancel_recovery_role_recovery_proposal");
    }
}
