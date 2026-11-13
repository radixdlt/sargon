use crate::prelude::*;
use radix_transactions::model::InstructionV2;

/// An opt-in, narrowly scoped exemption from the reserved-instruction rule for
/// **pre-authorization (subintent) manifests** sent by dApps.
///
/// Calling any method on an AccessController is a reserved instruction, so the
/// wallet refuses every dApp manifest that does it. That is the right default:
/// `create_proof` on a controller puts the controlled entity's owner badge on
/// the auth zone, and everything after it runs with owner authority.
///
/// It also rules out a legitimate use case. An account can be securified by an
/// AccessController that is **not** managed by this wallet: a multisig whose
/// primary role is a `CountOf` over signature badges held by several people.
/// Spending from such an account requires `create_proof` on its controller
/// *inside the subintent*. A proof minted in the parent transaction does not
/// authorize the child subintent. The external signing path
/// (`resolve_external_accounts_for_subintent`) already knows how to sign for
/// accounts like this. Only this rule stops the manifest from reaching it.
///
/// When the host opts in, the `AccessControllerMethod` reserved instruction is
/// waived for a manifest **only if all** of the following hold:
///
/// 1. Every AccessController method call is `create_proof`. Recovery, confirm,
///    lock/unlock, badge minting and all other controller methods stay reserved.
/// 2. Every invoked AccessController has a static address. Controllers
///    referenced through a named address are not exempt.
/// 3. No invoked AccessController secures an entity in this wallet's Profile.
///    The wallet's own Security Shields are never exempt.
/// 4. The manifest contains no role-assignment calls (`SET_OWNER_ROLE`,
///    `LOCK_OWNER_ROLE`, `SET_ROLE`, ...) on any entity. With an owner proof on
///    the auth zone these would silently and permanently change who controls
///    the account. The review screen does not show them as resource movements.
///
/// Every other reserved instruction (`lock_fee`, `securify`, owner-keys
/// metadata, ...) stays forbidden whether or not this exemption applies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExternalAccessControllerProofsPolicy {
    /// AccessControllers securing entities in the user's Profile, across all
    /// networks. Never exempt.
    pub access_controllers_of_profile_entities:
        IndexSet<AccessControllerAddress>,
}

impl ExternalAccessControllerProofsPolicy {
    pub fn new(
        access_controllers_of_profile_entities: impl IntoIterator<
            Item = AccessControllerAddress,
        >,
    ) -> Self {
        Self {
            access_controllers_of_profile_entities:
                access_controllers_of_profile_entities.into_iter().collect(),
        }
    }

    /// Returns `true` if the `AccessControllerMethod` reserved instruction may
    /// be waived for a manifest with these instructions, where
    /// `access_controller_receivers` are the AccessController receivers that
    /// RET's reserved-instruction analysis reported.
    pub(crate) fn permits(
        &self,
        instructions: &[InstructionV2],
        access_controller_receivers: &IndexSet<ScryptoManifestGlobalAddress>,
        network_id: NetworkID,
    ) -> bool {
        // (2) + (3): every receiver is a static, external AccessController.
        let mut receivers = IndexSet::<AccessControllerAddress>::new();
        for receiver in access_controller_receivers {
            let Ok(address) =
                AccessControllerAddress::try_from((*receiver, network_id))
            else {
                return false;
            };
            if self
                .access_controllers_of_profile_entities
                .contains(&address)
            {
                return false;
            }
            receivers.insert(address);
        }

        instructions.iter().all(|instruction| match instruction {
            // (1): only `create_proof` on any AccessController.
            InstructionV2::CallMethod(call) => {
                match AccessControllerAddress::try_from((
                    call.address,
                    network_id,
                )) {
                    Ok(address) => {
                        receivers.contains(&address)
                            && call.method_name
                                == SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT
                    }
                    // Not an AccessController. A named address could still
                    // refer to one, but then it would have been reported as a
                    // receiver above and failed (2).
                    Err(_) => true,
                }
            }
            // (4): no role-assignment changes anywhere.
            InstructionV2::CallRoleAssignmentMethod(_) => false,
            _ => true,
        })
    }
}

/// Reserved-instruction validation for pre-authorization (subintent)
/// manifests, which never originate from the host.
pub trait PreAuthorizationManifestValidation {
    /// Like [`StaticallyAnalyzableManifest::validated_summary`] with
    /// `are_instructions_originating_from_host: false`, but optionally applies
    /// [`ExternalAccessControllerProofsPolicy`].
    ///
    /// With `None`, behaviour is identical to `validated_summary(false)`.
    ///
    /// The returned summary is not modified: if the exemption was applied,
    /// `reserved_instructions` still contains `AccessControllerMethod`, so a
    /// host can tell the user that a multisig's authority is being used.
    fn validated_summary_for_pre_authorization(
        &self,
        external_access_controller_proofs: Option<
            &ExternalAccessControllerProofsPolicy,
        >,
    ) -> Result<ManifestSummary>;
}

impl PreAuthorizationManifestValidation for SubintentManifest {
    fn validated_summary_for_pre_authorization(
        &self,
        external_access_controller_proofs: Option<
            &ExternalAccessControllerProofsPolicy,
        >,
    ) -> Result<ManifestSummary> {
        let scrypto_manifest = self.scrypto_manifest();
        let analysis =
            RET_statically_analyze_subintent_manifest(&scrypto_manifest)
                .map_err(map_static_analysis_error)?;
        let access_controller_receivers = analysis
            .reserved_instructions_summary
            .access_controller_invocations
            .clone();
        let summary = ManifestSummary::from((analysis, self.network_id()));

        let exempt = external_access_controller_proofs.is_some_and(|policy| {
            summary
                .reserved_instructions
                .contains(&ReservedInstruction::AccessControllerMethod)
                && policy.permits(
                    &scrypto_manifest.instructions,
                    &access_controller_receivers,
                    self.network_id(),
                )
        });

        let forbidden = summary
            .reserved_instructions
            .iter()
            .filter(|instruction| {
                !(exempt
                    && **instruction
                        == ReservedInstruction::AccessControllerMethod)
            })
            .collect_vec();

        if !forbidden.is_empty() {
            return Err(
                CommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: forbidden
                        .iter()
                        .map(|i| i.to_string())
                        .collect(),
                },
            );
        }

        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExternalAccessControllerProofsPolicy;

    fn external_ac() -> AccessControllerAddress {
        AccessControllerAddress::sample_mainnet()
    }

    fn other_external_ac() -> AccessControllerAddress {
        AccessControllerAddress::sample_mainnet_other()
    }

    fn multisig_account() -> AccountAddress {
        AccountAddress::sample_mainnet()
    }

    fn recipient() -> AccountAddress {
        AccountAddress::sample_mainnet_other()
    }

    fn xrd() -> ResourceAddress {
        ResourceAddress::sample_mainnet_xrd()
    }

    fn manifest(body: impl AsRef<str>) -> SubintentManifest {
        SubintentManifest::new(
            format!("{}\nYIELD_TO_PARENT;\n", body.as_ref()),
            NetworkID::Mainnet,
            Blobs::default(),
            ChildSubintentSpecifiers::default(),
        )
        .unwrap()
    }

    fn create_proof(ac: AccessControllerAddress) -> String {
        format!(r#"CALL_METHOD Address("{ac}") "create_proof";"#)
    }

    fn spend() -> String {
        format!(
            r#"
CALL_METHOD Address("{account}") "withdraw" Address("{xrd}") Decimal("10");
CALL_METHOD Address("{recipient}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
"#,
            account = multisig_account(),
            xrd = xrd(),
            recipient = recipient(),
        )
    }

    /// The manifest shape a multisig coordinator sends: prove the controller's
    /// primary role, then spend from the account it controls.
    fn multisig_spend(ac: AccessControllerAddress) -> SubintentManifest {
        manifest(format!("{}\n{}", create_proof(ac), spend()))
    }

    fn rejected_reserved_instructions(
        result: Result<ManifestSummary>,
    ) -> String {
        match result {
            Err(CommonError::ReservedInstructionsNotAllowedInManifest {
                reserved_instructions,
            }) => reserved_instructions,
            other => panic!(
                "expected ReservedInstructionsNotAllowedInManifest, got {other:?}"
            ),
        }
    }

    #[test]
    fn create_proof_ident_is_create_proof() {
        assert_eq!(
            SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
            "create_proof"
        );
    }

    // ==================
    // Default behaviour is unchanged
    // ==================

    #[test]
    fn without_policy_create_proof_is_rejected() {
        let sut = multisig_spend(external_ac());
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(None),
        );
        assert_eq!(reserved, "AccessControllerMethod");
    }

    #[test]
    fn without_policy_matches_validated_summary_false() {
        let sut = multisig_spend(external_ac());
        assert_eq!(
            sut.validated_summary_for_pre_authorization(None),
            sut.validated_summary(false),
        );

        let plain = manifest(spend());
        assert_eq!(
            plain.validated_summary_for_pre_authorization(None),
            plain.validated_summary(false),
        );
    }

    #[test]
    fn manifest_without_access_controller_is_unaffected_by_policy() {
        let sut = manifest(spend());
        let policy = SUT::default();
        assert!(sut
            .validated_summary_for_pre_authorization(Some(&policy))
            .is_ok());
        assert_eq!(
            sut.validated_summary_for_pre_authorization(Some(&policy)),
            sut.validated_summary(false),
        );
    }

    // ==================
    // The exemption
    // ==================

    #[test]
    fn policy_permits_create_proof_on_external_access_controller() {
        let sut = multisig_spend(external_ac());
        let summary = sut
            .validated_summary_for_pre_authorization(Some(&SUT::default()))
            .unwrap();

        // The summary is not rewritten, so the host can still tell the user
        // that an AccessController's authority is used.
        assert_eq!(
            summary.reserved_instructions,
            vec![ReservedInstruction::AccessControllerMethod]
        );
        assert_eq!(
            summary.addresses_of_accounts_requiring_auth,
            vec![multisig_account()]
        );
        assert_eq!(summary, sut.summary().unwrap());
    }

    #[test]
    fn policy_permits_create_proof_on_several_external_access_controllers() {
        let sut = manifest(format!(
            "{}\n{}\n{}",
            create_proof(external_ac()),
            create_proof(other_external_ac()),
            spend()
        ));
        assert!(sut
            .validated_summary_for_pre_authorization(Some(&SUT::default()))
            .is_ok());
    }

    #[test]
    fn policy_permits_create_proof_when_other_controllers_are_in_profile() {
        let sut = multisig_spend(external_ac());
        let policy = SUT::new([other_external_ac()]);
        assert!(sut
            .validated_summary_for_pre_authorization(Some(&policy))
            .is_ok());
    }

    // ==================
    // (3) The wallet's own controllers are never exempt
    // ==================

    #[test]
    fn policy_rejects_access_controller_of_profile_entity() {
        let sut = multisig_spend(external_ac());
        let policy = SUT::new([external_ac()]);
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(Some(&policy)),
        );
        assert_eq!(reserved, "AccessControllerMethod");
    }

    #[test]
    fn policy_rejects_if_any_access_controller_belongs_to_profile() {
        let sut = manifest(format!(
            "{}\n{}\n{}",
            create_proof(external_ac()),
            create_proof(other_external_ac()),
            spend()
        ));
        let policy = SUT::new([other_external_ac()]);
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(Some(&policy)),
        );
        assert_eq!(reserved, "AccessControllerMethod");
    }

    // ==================
    // (1) Only `create_proof`
    // ==================

    #[test]
    fn policy_rejects_every_other_access_controller_method() {
        let ac = external_ac();
        let rule_set = "Tuple(Enum<0u8>(), Enum<0u8>(), Enum<0u8>())";
        // Every other method on the AccessController blueprint, with valid
        // arguments so that RET recognises each call as an AC invocation.
        let calls = [
            format!(r#"CALL_METHOD Address("{ac}") "lock_primary_role";"#),
            format!(r#"CALL_METHOD Address("{ac}") "unlock_primary_role";"#),
            format!(
                r#"CALL_METHOD Address("{ac}") "stop_timed_recovery" {rule_set} None;"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "initiate_recovery_as_primary" {rule_set} None;"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "initiate_recovery_as_recovery" {rule_set} None;"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "quick_confirm_primary_role_recovery_proposal" {rule_set} None;"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "timed_confirm_recovery" {rule_set} None;"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "cancel_primary_role_recovery_proposal";"#
            ),
            format!(
                r#"CALL_METHOD Address("{ac}") "mint_recovery_badges" Array<NonFungibleLocalId>();"#
            ),
        ];
        for call in calls {
            let sut = manifest(format!("{call}\n{}", spend()));
            assert!(
                sut.summary()
                    .unwrap()
                    .reserved_instructions
                    .contains(&ReservedInstruction::AccessControllerMethod),
                "RET did not flag `{call}` as reserved, so this case tests nothing"
            );
            let reserved = rejected_reserved_instructions(
                sut.validated_summary_for_pre_authorization(Some(
                    &SUT::default(),
                )),
            );
            assert_eq!(reserved, "AccessControllerMethod", "for `{call}`");
        }
    }

    #[test]
    fn policy_rejects_create_proof_combined_with_another_method() {
        let ac = external_ac();
        let sut = manifest(format!(
            r#"{}
CALL_METHOD Address("{ac}") "lock_primary_role";
{}"#,
            create_proof(ac),
            spend()
        ));
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(Some(&SUT::default())),
        );
        assert_eq!(reserved, "AccessControllerMethod");
    }

    // ==================
    // (4) No role-assignment changes
    // ==================

    #[test]
    fn policy_rejects_role_changes_after_create_proof() {
        let account = multisig_account();
        let attacks = [
            format!(r#"SET_OWNER_ROLE Address("{account}") Enum<0u8>();"#),
            format!(r#"LOCK_OWNER_ROLE Address("{account}");"#),
            format!(
                r#"SET_ROLE Address("{account}") Enum<0u8>() "securify" Enum<0u8>();"#
            ),
        ];
        for attack in attacks {
            let sut = manifest(format!(
                "{}\n{attack}\n{}",
                create_proof(external_ac()),
                spend()
            ));
            let reserved = rejected_reserved_instructions(
                sut.validated_summary_for_pre_authorization(Some(
                    &SUT::default(),
                )),
            );
            assert_eq!(reserved, "AccessControllerMethod", "for `{attack}`");
        }
    }

    #[test]
    fn role_change_without_access_controller_keeps_todays_behaviour() {
        // Not reserved today. The policy never widens or narrows anything
        // outside its own exemption.
        let account = multisig_account();
        let sut = manifest(format!(
            r#"SET_OWNER_ROLE Address("{account}") Enum<0u8>();
{}"#,
            spend()
        ));
        assert_eq!(
            sut.validated_summary_for_pre_authorization(Some(&SUT::default())),
            sut.validated_summary(false),
        );
    }

    // ==================
    // Other reserved instructions stay forbidden
    // ==================

    #[test]
    fn policy_keeps_lock_fee_forbidden() {
        let account = multisig_account();
        let sut = manifest(format!(
            r#"{}
CALL_METHOD Address("{account}") "lock_fee" Decimal("1");
{}"#,
            create_proof(external_ac()),
            spend()
        ));
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(Some(&SUT::default())),
        );
        // The AccessController call is exempt; lock_fee is not.
        assert_eq!(reserved, "AccountLockFee");
    }

    #[test]
    fn policy_keeps_securify_forbidden() {
        let account = multisig_account();
        let sut = manifest(format!(
            r#"{}
CALL_METHOD Address("{account}") "securify";
{}"#,
            create_proof(external_ac()),
            spend()
        ));
        let reserved = rejected_reserved_instructions(
            sut.validated_summary_for_pre_authorization(Some(&SUT::default())),
        );
        assert_eq!(reserved, "AccountSecurify");
    }
}
