use crate::prelude::*;

pub(crate) trait OsValidatePreAuthAccessControllerCalls {
    fn validate_pre_auth_access_controller_calls(
        &self,
        manifest: &SubintentManifest,
        summary: &ManifestSummary,
        execution_summary: &ExecutionSummary,
        access_controllers_on_ledger: &IndexMap<
            AccessControllerAddress,
            AccessControllerStateDetails,
        >,
    ) -> Result<()>;
}

impl OsValidatePreAuthAccessControllerCalls for SargonOS {
    /// Allows the access controller calls of a pre-authorization only on
    /// access controllers the user controls together with others, see
    /// [`shared_control_refusal`], with their current rules from
    /// `access_controllers_on_ledger`, as read for the review. Signing checks
    /// the same again.
    ///
    /// Errors with the reserved instructions otherwise.
    fn validate_pre_auth_access_controller_calls(
        &self,
        manifest: &SubintentManifest,
        summary: &ManifestSummary,
        execution_summary: &ExecutionSummary,
        access_controllers_on_ledger: &IndexMap<
            AccessControllerAddress,
            AccessControllerStateDetails,
        >,
    ) -> Result<()> {
        let Some(reason) = shared_control_refusal(
            execution_summary.detailed_classification.as_ref(),
            summary,
            access_controllers_on_ledger,
            self.mfa_signing_keys()?,
            manifest.network_id(),
        ) else {
            return Ok(());
        };
        warn!("Pre-auth access controller calls rejected: {reason}");
        Err(CommonError::ReservedInstructionsNotAllowedInManifest {
            reserved_instructions: summary
                .reserved_instructions
                .iter()
                .map(|i| i.to_string())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine_interface::prelude::{require, require_n_of, AccessRule};
    use ScryptoNonFungibleGlobalId as Badge;

    const NETWORK: NetworkID = NetworkID::Stokenet;

    fn badge(byte: u8) -> Badge {
        Badge::from_public_key(ScryptoEd25519PublicKey([byte; 32]))
    }

    /// The user's badge is 1, the others' are 2 and 3.
    fn user_alone() -> AuthZone {
        AuthZone {
            implicit_non_fungibles: [NonFungibleGlobalId::from((
                badge(1),
                NETWORK,
            ))]
            .into(),
            proofs: Vec::new(),
        }
    }

    fn shared() -> AccessRule {
        AccessRule::Protected(require_n_of(
            2u8,
            vec![badge(1), badge(2), badge(3)],
        ))
    }

    fn rule_sets(
        roles: [AccessRule; 3],
    ) -> IndexMap<AccessControllerAddress, AccessControllerRuleSet> {
        let [primary_role, recovery_role, confirmation_role] = roles;
        IndexMap::from([(
            AccessControllerAddress::sample_stokenet(),
            (
                ScryptoRuleSet {
                    primary_role,
                    recovery_role,
                    confirmation_role,
                },
                NETWORK,
            )
                .into(),
        )])
    }

    fn called() -> IndexSet<AccessControllerAddress> {
        IndexSet::from([AccessControllerAddress::sample_stokenet()])
    }

    #[test]
    fn access_controller_operations_are_allowed_but_not_moving_xrd() {
        let allowed = [
            DetailedManifestClass::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: Vec::new(),
            },
            DetailedManifestClass::AccessControllerTimedRecovery {
                ac_addresses: Vec::new(),
                timed_recoveries: Vec::new(),
            },
            DetailedManifestClass::AccessControllerConfirmRecovery {
                ac_addresses: Vec::new(),
                confirmations: Vec::new(),
            },
            DetailedManifestClass::AccessControllerCancelRecovery {
                ac_addresses: Vec::new(),
                cancellations: Vec::new(),
            },
            DetailedManifestClass::AccessControllerPrimaryRoleLock {
                ac_addresses: Vec::new(),
                operations: Vec::new(),
            },
        ];
        let not_allowed = [
            DetailedManifestClass::AccessControllerRecoveryFee {
                ac_addresses: Vec::new(),
                contributions: Vec::new(),
                withdrawals: Vec::new(),
                account_withdrawals: Vec::new(),
                account_deposits: Vec::new(),
            },
            DetailedManifestClass::SecurifyEntity {
                entities: Vec::new(),
            },
            DetailedManifestClass::General,
        ];
        assert!(allowed.iter().all(is_allowed_in_pre_authorizations));
        assert!(!not_allowed.iter().any(is_allowed_in_pre_authorizations));
    }

    /// "Any 2 of these 3" where one key is listed twice.
    fn rules_counting_a_key_twice() -> AccessControllerRuleSetForDisplay {
        let shared_rules = || -> AccessControllerRuleSet {
            (
                ScryptoRuleSet {
                    primary_role: shared(),
                    recovery_role: shared(),
                    confirmation_role: shared(),
                },
                NETWORK,
            )
                .into()
        };
        let counted_twice = AccessRule::Protected(require_n_of(
            2u8,
            vec![badge(2), badge(2), badge(1)],
        ));
        assert!(!shared_rules().for_display().has_overlapping_entries());
        AccessControllerRuleSet {
            recovery_role: (counted_twice, NETWORK).into(),
            ..shared_rules()
        }
        .for_display()
    }

    #[test]
    fn rules_to_enact_must_not_count_a_key_twice() {
        let proposed_rules = rules_counting_a_key_twice();
        let enacting = [
            DetailedManifestClass::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: vec![AccessControllerRecoveryProposal {
                    proposed_rules: proposed_rules.clone(),
                    ..AccessControllerRecoveryProposal::sample()
                }],
            },
            DetailedManifestClass::AccessControllerTimedRecovery {
                ac_addresses: Vec::new(),
                timed_recoveries: vec![AccessControllerTimedRecoveryProposal {
                    proposed_rules: proposed_rules.clone(),
                    ..AccessControllerTimedRecoveryProposal::sample()
                }],
            },
            DetailedManifestClass::AccessControllerConfirmRecovery {
                ac_addresses: Vec::new(),
                confirmations: vec![AccessControllerRecoveryConfirmation {
                    proposed_rules: proposed_rules.clone(),
                    ..AccessControllerRecoveryConfirmation::sample()
                }],
            },
        ];
        assert!(enacting.iter().all(enacts_rules_with_overlapping_entries));

        let not_enacting = [
            DetailedManifestClass::AccessControllerRecovery {
                ac_addresses: Vec::new(),
                recovery_proposals: vec![
                    AccessControllerRecoveryProposal::sample(),
                ],
            },
            // Stopping such a proposal must stay possible.
            DetailedManifestClass::AccessControllerCancelRecovery {
                ac_addresses: Vec::new(),
                cancellations: vec![AccessControllerRecoveryCancellation {
                    stopped_timed_recovery: Some(proposed_rules),
                    ..AccessControllerRecoveryCancellation::sample()
                }],
            },
        ];
        assert!(!not_enacting
            .iter()
            .any(enacts_rules_with_overlapping_entries));
    }

    #[test]
    fn roles_shared_with_others_cannot_be_passed_alone() {
        assert!(!can_pass_any_role_alone(
            &called(),
            &rule_sets([shared(), shared(), shared()]),
            &user_alone()
        ));
    }

    #[test]
    fn any_role_the_user_holds_alone_is_passed_alone() {
        let alone = || AccessRule::Protected(require(badge(1)));
        for roles in [
            [alone(), shared(), shared()],
            [shared(), alone(), shared()],
            [shared(), shared(), alone()],
        ] {
            assert!(can_pass_any_role_alone(
                &called(),
                &rule_sets(roles),
                &user_alone()
            ));
        }
    }

    /// Sargon's override-only roles pass with no proof at all.
    #[test]
    fn a_role_anyone_passes_is_passed_alone() {
        assert!(can_pass_any_role_alone(
            &called(),
            &rule_sets([shared(), AccessRule::AllowAll, shared()]),
            &user_alone()
        ));
    }

    #[test]
    fn an_access_controller_without_known_rules_counts_as_passed_alone() {
        assert!(can_pass_any_role_alone(
            &called(),
            &IndexMap::new(),
            &user_alone()
        ));
    }
}
