use crate::prelude::*;
use radix_common::data::manifest::{manifest_decode, manifest_encode};
use radix_common::prelude::{ManifestDecode, ManifestValue};
use radix_engine_interface::blueprints::resource::{
    BasicRequirement as ScryptoBasicRequirement,
    CompositeRequirement as ScryptoCompositeRequirement,
    ResourceOrNonFungible as ScryptoResourceOrNonFungible,
};
use radix_transactions::model::InstructionV2;

// ==================
// Decoded AccessController operations
// ==================

/// A simplified view of one AccessController role rule, for showing a
/// proposed rule set to the user.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AccessControllerRoleRule {
    /// Anyone can act in this role.
    AllowAll,

    /// Nobody can act in this role.
    DenyAll,

    /// At least `threshold` of `badges` must be proven.
    ///
    /// `require(x)` is `1` of `[x]`, `any_of(l)` is `1` of `l` and `all_of(l)`
    /// is `l.len()` of `l`.
    Threshold {
        threshold: u8,
        badges: Vec<ResourceOrNonFungible>,
    },

    /// A rule this summary cannot express: an amount requirement, or a nested
    /// combination of requirements. Hosts should flag it to the user.
    Complex,
}

impl AccessControllerRoleRule {
    fn from_scrypto(rule: ScryptoAccessRule, network_id: NetworkID) -> Self {
        let threshold_of =
            |threshold: usize, list: Vec<ScryptoResourceOrNonFungible>| {
                match u8::try_from(threshold) {
                    Ok(threshold) => Self::Threshold {
                        threshold,
                        badges: list
                            .into_iter()
                            .map(|b| {
                                ResourceOrNonFungible::from((b, network_id))
                            })
                            .collect(),
                    },
                    Err(_) => Self::Complex,
                }
            };
        match rule {
            ScryptoAccessRule::AllowAll => Self::AllowAll,
            ScryptoAccessRule::DenyAll => Self::DenyAll,
            ScryptoAccessRule::Protected(
                ScryptoCompositeRequirement::BasicRequirement(basic),
            ) => match basic {
                ScryptoBasicRequirement::Require(badge) => {
                    threshold_of(1, vec![badge])
                }
                ScryptoBasicRequirement::CountOf(threshold, list) => {
                    threshold_of(threshold as usize, list)
                }
                ScryptoBasicRequirement::AnyOf(list) => threshold_of(1, list),
                ScryptoBasicRequirement::AllOf(list) => {
                    threshold_of(list.len(), list)
                }
                ScryptoBasicRequirement::AmountOf(..) => Self::Complex,
            },
            ScryptoAccessRule::Protected(_) => Self::Complex,
        }
    }
}

/// The rule set carried by a recovery-family call: proposed by `initiate_*`,
/// and repeated by `quick_confirm_*`, `timed_confirm_recovery` and
/// `stop_timed_recovery` to name the proposal they act on.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRuleSetProposal {
    pub primary: AccessControllerRoleRule,
    pub recovery: AccessControllerRoleRule,
    pub confirmation: AccessControllerRoleRule,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

/// What one AccessController call in a manifest does.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AccessControllerOperationKind {
    /// Proves the primary role and puts the controlled entity's owner badge
    /// on the auth zone.
    CreateProof,
    /// Proposes a new rule set, signed by the primary role.
    InitiateRecoveryAsPrimary {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Proposes a new rule set, signed by the recovery role.
    InitiateRecoveryAsRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Confirms a rule set the primary role proposed.
    QuickConfirmPrimaryRoleRecoveryProposal {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Confirms a rule set the recovery role proposed.
    QuickConfirmRecoveryRoleRecoveryProposal {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Enacts a recovery-role proposal after its timed delay.
    TimedConfirmRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Stops a timed recovery in progress.
    StopTimedRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    /// Withdraws the primary role's pending proposal.
    CancelPrimaryRoleRecoveryProposal,
    /// Withdraws the recovery role's pending proposal.
    CancelRecoveryRoleRecoveryProposal,
    /// Freezes the primary role: spending stops until unlocked.
    LockPrimaryRole,
    /// Lifts a primary-role freeze.
    UnlockPrimaryRole,
    /// Any other method, or a supported method whose arguments could not be
    /// decoded. Never exempt from the reserved-instruction rule.
    Unsupported { method_name: String },
}

impl AccessControllerOperationKind {
    /// `false` only for `Unsupported`.
    pub fn is_supported(&self) -> bool {
        !matches!(self, Self::Unsupported { .. })
    }
}

/// One AccessController call in a manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerOperation {
    pub access_controller: AccessControllerAddress,
    pub kind: AccessControllerOperationKind,
}

#[derive(ScryptoManifestSbor)]
struct DecodedNoArgs {}

#[derive(ScryptoManifestSbor)]
struct DecodedRuleSet {
    primary_role: ScryptoAccessRule,
    recovery_role: ScryptoAccessRule,
    confirmation_role: ScryptoAccessRule,
}

#[derive(ScryptoManifestSbor)]
struct DecodedRecoveryArgs {
    rule_set: DecodedRuleSet,
    timed_recovery_delay_in_minutes: Option<u32>,
}

fn decode_args<T: ManifestDecode>(args: &ManifestValue) -> Option<T> {
    let bytes = manifest_encode(args).ok()?;
    manifest_decode::<T>(&bytes).ok()
}

impl AccessControllerOperationKind {
    fn decode(
        method_name: &str,
        args: &ManifestValue,
        network_id: NetworkID,
    ) -> Self {
        let no_args =
            |kind: Self| decode_args::<DecodedNoArgs>(args).map(|_| kind);
        let proposal = || {
            decode_args::<DecodedRecoveryArgs>(args).map(|decoded| {
                AccessControllerRuleSetProposal {
                    primary: AccessControllerRoleRule::from_scrypto(
                        decoded.rule_set.primary_role,
                        network_id,
                    ),
                    recovery: AccessControllerRoleRule::from_scrypto(
                        decoded.rule_set.recovery_role,
                        network_id,
                    ),
                    confirmation: AccessControllerRoleRule::from_scrypto(
                        decoded.rule_set.confirmation_role,
                        network_id,
                    ),
                    timed_recovery_delay_in_minutes: decoded
                        .timed_recovery_delay_in_minutes,
                }
            })
        };

        let decoded = match method_name {
            "create_proof" => no_args(Self::CreateProof),
            "initiate_recovery_as_primary" => proposal()
                .map(|proposal| Self::InitiateRecoveryAsPrimary { proposal }),
            "initiate_recovery_as_recovery" => proposal()
                .map(|proposal| Self::InitiateRecoveryAsRecovery { proposal }),
            "quick_confirm_primary_role_recovery_proposal" => {
                proposal().map(|proposal| {
                    Self::QuickConfirmPrimaryRoleRecoveryProposal { proposal }
                })
            }
            "quick_confirm_recovery_role_recovery_proposal" => {
                proposal().map(|proposal| {
                    Self::QuickConfirmRecoveryRoleRecoveryProposal { proposal }
                })
            }
            "timed_confirm_recovery" => proposal()
                .map(|proposal| Self::TimedConfirmRecovery { proposal }),
            "stop_timed_recovery" => {
                proposal().map(|proposal| Self::StopTimedRecovery { proposal })
            }
            "cancel_primary_role_recovery_proposal" => {
                no_args(Self::CancelPrimaryRoleRecoveryProposal)
            }
            "cancel_recovery_role_recovery_proposal" => {
                no_args(Self::CancelRecoveryRoleRecoveryProposal)
            }
            "lock_primary_role" => no_args(Self::LockPrimaryRole),
            "unlock_primary_role" => no_args(Self::UnlockPrimaryRole),
            _ => None,
        };

        decoded.unwrap_or_else(|| Self::Unsupported {
            method_name: method_name.to_owned(),
        })
    }
}

/// Static decoding of the AccessController calls in a manifest.
pub trait AccessControllerOperationsAnalysis {
    /// Every `CALL_METHOD` on a statically addressed AccessController, in
    /// manifest order. Calls through a named address are not included; RET
    /// still reports them as reserved, and the exemption rejects them.
    fn access_controller_operations(&self) -> Vec<AccessControllerOperation>;
}

impl AccessControllerOperationsAnalysis for SubintentManifest {
    fn access_controller_operations(&self) -> Vec<AccessControllerOperation> {
        let network_id = self.network_id();
        self.scrypto_manifest()
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                InstructionV2::CallMethod(call) => {
                    AccessControllerAddress::try_from((
                        call.address,
                        network_id,
                    ))
                    .ok()
                    .map(|access_controller| {
                        AccessControllerOperation {
                            access_controller,
                            kind: AccessControllerOperationKind::decode(
                                &call.method_name,
                                &call.args,
                                network_id,
                            ),
                        }
                    })
                }
                _ => None,
            })
            .collect()
    }
}

// ==================
// The exemption
// ==================

/// An opt-in, narrowly scoped exemption from the reserved-instruction rule for
/// **pre-authorization (subintent) manifests** sent by dApps.
///
/// Calling any method on an AccessController is a reserved instruction, so the
/// wallet refuses every dApp manifest that does it. That is the right default
/// for the wallet's own Security Shields: `create_proof` puts the controlled
/// entity's owner badge on the auth zone, and recovery calls change who
/// controls the entity.
///
/// It also rules out multisig accounts secured by an AccessController that is
/// **not** managed by this wallet, whose roles are `CountOf` rules over
/// signature badges held by several people. Operating such an account means
/// calling its controller *inside the subintent*: `create_proof` to spend,
/// recovery calls to rotate signers, `lock_primary_role` to freeze. A call in
/// the parent transaction does not authorize the child subintent.
///
/// When the host opts in, the `AccessControllerMethod` reserved instruction is
/// waived for a manifest **only if all** of the following hold:
///
/// 1. Every AccessController call is a supported operation (see
///    [`AccessControllerOperationKind`]) with decodable arguments. Badge
///    withdrawal, recovery-badge minting and the recovery-fee vault methods
///    stay reserved.
/// 2. Every invoked AccessController has a static address.
/// 3. No invoked AccessController secures an entity in this wallet's Profile.
///    The wallet's own Security Shields are never exempt.
/// 4. The manifest contains no role-assignment calls (`SET_OWNER_ROLE`,
///    `LOCK_OWNER_ROLE`, `SET_ROLE`, ...) on any entity. With an owner proof on
///    the auth zone these would silently and permanently change who controls
///    the account.
///
/// Every other reserved instruction (`lock_fee`, `securify`, owner-keys
/// metadata, ...) stays forbidden whether or not this exemption applies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExternalAccessControllerPolicy {
    /// AccessControllers securing entities in the user's Profile, across all
    /// networks. Never exempt.
    pub access_controllers_of_profile_entities:
        IndexSet<AccessControllerAddress>,
}

impl ExternalAccessControllerPolicy {
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
    /// be waived for `manifest`, where `access_controller_receivers` are the
    /// AccessController receivers that RET's reserved-instruction analysis
    /// reported.
    pub(crate) fn permits(
        &self,
        manifest: &SubintentManifest,
        access_controller_receivers: &IndexSet<ScryptoManifestGlobalAddress>,
    ) -> bool {
        let network_id = manifest.network_id();

        // (2) + (3): every receiver is a static, external AccessController.
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
        }

        // (1): every AccessController call is a supported operation.
        let operations = manifest.access_controller_operations();
        if operations.is_empty()
            || !operations.iter().all(|op| op.kind.is_supported())
        {
            return false;
        }

        // (4): no role-assignment changes anywhere.
        !manifest
            .scrypto_manifest()
            .instructions
            .iter()
            .any(|i| matches!(i, InstructionV2::CallRoleAssignmentMethod(_)))
    }
}

/// Reserved-instruction validation for pre-authorization (subintent)
/// manifests, which never originate from the host.
pub trait PreAuthorizationManifestValidation {
    /// Like [`StaticallyAnalyzableManifest::validated_summary`] with
    /// `are_instructions_originating_from_host: false`, but optionally applies
    /// [`ExternalAccessControllerPolicy`].
    ///
    /// With `None`, behaviour is identical to `validated_summary(false)`.
    ///
    /// The returned summary is not modified: if the exemption was applied,
    /// `reserved_instructions` still contains `AccessControllerMethod`, so a
    /// host can tell the user that a multisig's controller is being operated.
    /// [`AccessControllerOperationsAnalysis::access_controller_operations`]
    /// says what each call does.
    fn validated_summary_for_pre_authorization(
        &self,
        external_access_controllers: Option<&ExternalAccessControllerPolicy>,
    ) -> Result<ManifestSummary>;
}

impl PreAuthorizationManifestValidation for SubintentManifest {
    fn validated_summary_for_pre_authorization(
        &self,
        external_access_controllers: Option<&ExternalAccessControllerPolicy>,
    ) -> Result<ManifestSummary> {
        let analysis =
            RET_statically_analyze_subintent_manifest(&self.scrypto_manifest())
                .map_err(map_static_analysis_error)?;
        let access_controller_receivers = analysis
            .reserved_instructions_summary
            .access_controller_invocations
            .clone();
        let summary = ManifestSummary::from((analysis, self.network_id()));

        let exempt = external_access_controllers.is_some_and(|policy| {
            summary
                .reserved_instructions
                .contains(&ReservedInstruction::AccessControllerMethod)
                && policy.permits(self, &access_controller_receivers)
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

// ==================
// Samples
// ==================

impl HasSampleValues for AccessControllerRoleRule {
    fn sample() -> Self {
        Self::Threshold {
            threshold: 2,
            badges: vec![
                ResourceOrNonFungible::sample(),
                ResourceOrNonFungible::sample_other(),
            ],
        }
    }

    fn sample_other() -> Self {
        Self::AllowAll
    }
}

impl HasSampleValues for AccessControllerRuleSetProposal {
    fn sample() -> Self {
        Self {
            primary: AccessControllerRoleRule::sample(),
            recovery: AccessControllerRoleRule::Threshold {
                threshold: 1,
                badges: vec![ResourceOrNonFungible::sample()],
            },
            confirmation: AccessControllerRoleRule::sample_other(),
            timed_recovery_delay_in_minutes: Some(20160),
        }
    }

    fn sample_other() -> Self {
        Self {
            primary: AccessControllerRoleRule::DenyAll,
            recovery: AccessControllerRoleRule::Complex,
            confirmation: AccessControllerRoleRule::AllowAll,
            timed_recovery_delay_in_minutes: None,
        }
    }
}

impl HasSampleValues for AccessControllerOperationKind {
    fn sample() -> Self {
        Self::InitiateRecoveryAsPrimary {
            proposal: AccessControllerRuleSetProposal::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::LockPrimaryRole
    }
}

impl HasSampleValues for AccessControllerOperation {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            kind: AccessControllerOperationKind::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            kind: AccessControllerOperationKind::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExternalAccessControllerPolicy;

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

    fn signer_a() -> NonFungibleGlobalId {
        NonFungibleGlobalId::sample()
    }

    fn signer_b() -> NonFungibleGlobalId {
        NonFungibleGlobalId::sample_other()
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

    /// The manifest shape a multisig coordinator sends to spend: prove the
    /// controller's primary role, then spend from the account it controls.
    fn multisig_spend(ac: AccessControllerAddress) -> SubintentManifest {
        manifest(format!("{}\n{}", create_proof(ac), spend()))
    }

    /// The rule set Signetix proposes when rotating signers: primary
    /// `2 of [a, b]`, recovery `require(a)`, confirmation `allow_all`, and a
    /// 14-day timed delay. Encoded exactly as Signetix encodes it.
    fn rule_set_args() -> String {
        let (a, b) = (signer_a(), signer_b());
        format!(
            r#"Tuple(
    Enum<2u8>(Enum<0u8>(Enum<2u8>(2u8, Array<Enum>(
        Enum<0u8>(NonFungibleGlobalId("{a}")),
        Enum<0u8>(NonFungibleGlobalId("{b}"))
    )))),
    Enum<2u8>(Enum<0u8>(Enum<0u8>(Enum<0u8>(NonFungibleGlobalId("{a}"))))),
    Enum<0u8>()
)
Enum<1u8>(20160u32)"#
        )
    }

    fn expected_proposal() -> AccessControllerRuleSetProposal {
        let nf = |value| ResourceOrNonFungible::NonFungible { value };
        AccessControllerRuleSetProposal {
            primary: AccessControllerRoleRule::Threshold {
                threshold: 2,
                badges: vec![nf(signer_a()), nf(signer_b())],
            },
            recovery: AccessControllerRoleRule::Threshold {
                threshold: 1,
                badges: vec![nf(signer_a())],
            },
            confirmation: AccessControllerRoleRule::AllowAll,
            timed_recovery_delay_in_minutes: Some(20160),
        }
    }

    /// Every supported controller call as Signetix sends it: one call, then
    /// `YIELD_TO_PARENT`. Paired with the operation it must decode to.
    fn supported_calls(
        ac: AccessControllerAddress,
    ) -> Vec<(String, AccessControllerOperationKind)> {
        use AccessControllerOperationKind as K;
        let rs = rule_set_args();
        let p = expected_proposal;
        vec![
            (create_proof(ac), K::CreateProof),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "initiate_recovery_as_primary" {rs};"#
                ),
                K::InitiateRecoveryAsPrimary { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "initiate_recovery_as_recovery" {rs};"#
                ),
                K::InitiateRecoveryAsRecovery { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "quick_confirm_primary_role_recovery_proposal" {rs};"#
                ),
                K::QuickConfirmPrimaryRoleRecoveryProposal { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "quick_confirm_recovery_role_recovery_proposal" {rs};"#
                ),
                K::QuickConfirmRecoveryRoleRecoveryProposal { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "timed_confirm_recovery" {rs};"#
                ),
                K::TimedConfirmRecovery { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "stop_timed_recovery" {rs};"#
                ),
                K::StopTimedRecovery { proposal: p() },
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "cancel_primary_role_recovery_proposal";"#
                ),
                K::CancelPrimaryRoleRecoveryProposal,
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "cancel_recovery_role_recovery_proposal";"#
                ),
                K::CancelRecoveryRoleRecoveryProposal,
            ),
            (
                format!(r#"CALL_METHOD Address("{ac}") "lock_primary_role";"#),
                K::LockPrimaryRole,
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "unlock_primary_role";"#
                ),
                K::UnlockPrimaryRole,
            ),
        ]
    }

    /// Every other AccessController method, with valid arguments so that RET
    /// recognises each call as an AccessController invocation.
    fn unsupported_calls(
        ac: AccessControllerAddress,
    ) -> Vec<(String, &'static str)> {
        vec![
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "mint_recovery_badges" Array<NonFungibleLocalId>();"#
                ),
                "mint_recovery_badges",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "lock_recovery_fee" Decimal("1");"#
                ),
                "lock_recovery_fee",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "withdraw_recovery_fee" Decimal("1");"#
                ),
                "withdraw_recovery_fee",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "initiate_badge_withdraw_attempt_as_primary";"#
                ),
                "initiate_badge_withdraw_attempt_as_primary",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "initiate_badge_withdraw_attempt_as_recovery";"#
                ),
                "initiate_badge_withdraw_attempt_as_recovery",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "quick_confirm_primary_role_badge_withdraw_attempt";"#
                ),
                "quick_confirm_primary_role_badge_withdraw_attempt",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "quick_confirm_recovery_role_badge_withdraw_attempt";"#
                ),
                "quick_confirm_recovery_role_badge_withdraw_attempt",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "cancel_primary_role_badge_withdraw_attempt";"#
                ),
                "cancel_primary_role_badge_withdraw_attempt",
            ),
            (
                format!(
                    r#"CALL_METHOD Address("{ac}") "cancel_recovery_role_badge_withdraw_attempt";"#
                ),
                "cancel_recovery_role_badge_withdraw_attempt",
            ),
        ]
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

    fn assert_flagged_by_ret(sut: &SubintentManifest, what: &str) {
        assert!(
            sut.summary()
                .unwrap()
                .reserved_instructions
                .contains(&ReservedInstruction::AccessControllerMethod),
            "RET did not flag `{what}` as reserved, so this case tests nothing"
        );
    }

    // ==================
    // Decoding
    // ==================

    #[test]
    fn decodes_every_supported_operation() {
        for (call, expected) in supported_calls(external_ac()) {
            let sut = manifest(&call);
            assert_eq!(
                sut.access_controller_operations(),
                vec![AccessControllerOperation {
                    access_controller: external_ac(),
                    kind: expected,
                }],
                "for `{call}`"
            );
        }
    }

    #[test]
    fn decodes_every_other_method_as_unsupported() {
        for (call, method_name) in unsupported_calls(external_ac()) {
            let sut = manifest(&call);
            assert_eq!(
                sut.access_controller_operations(),
                vec![AccessControllerOperation {
                    access_controller: external_ac(),
                    kind: AccessControllerOperationKind::Unsupported {
                        method_name: method_name.to_owned()
                    },
                }],
                "for `{call}`"
            );
        }
    }

    #[test]
    fn supported_method_with_wrong_arguments_is_unsupported() {
        let ac = external_ac();
        let sut = manifest(format!(
            r#"CALL_METHOD Address("{ac}") "lock_primary_role" Decimal("1");"#
        ));
        assert_eq!(
            sut.access_controller_operations(),
            vec![AccessControllerOperation {
                access_controller: ac,
                kind: AccessControllerOperationKind::Unsupported {
                    method_name: "lock_primary_role".to_owned()
                },
            }]
        );
    }

    #[test]
    fn decodes_operations_in_manifest_order_and_ignores_other_calls() {
        let sut = manifest(format!(
            "{}\n{}\n{}",
            create_proof(other_external_ac()),
            spend(),
            create_proof(external_ac()),
        ));
        assert_eq!(
            sut.access_controller_operations()
                .into_iter()
                .map(|op| op.access_controller)
                .collect_vec(),
            vec![other_external_ac(), external_ac()]
        );
    }

    #[test]
    fn role_rule_shapes() {
        let network_id = NetworkID::Mainnet;
        let a = ScryptoNonFungibleGlobalId::from(signer_a());
        let b = ScryptoNonFungibleGlobalId::from(signer_b());
        let ron = |id: &ScryptoNonFungibleGlobalId| {
            ScryptoResourceOrNonFungible::NonFungible(id.clone())
        };
        let basic = |requirement| {
            ScryptoAccessRule::Protected(
                ScryptoCompositeRequirement::BasicRequirement(requirement),
            )
        };
        let nf = |value| ResourceOrNonFungible::NonFungible { value };
        let threshold = |threshold, badges| {
            AccessControllerRoleRule::Threshold { threshold, badges }
        };
        let cases = [
            (
                ScryptoAccessRule::AllowAll,
                AccessControllerRoleRule::AllowAll,
            ),
            (
                ScryptoAccessRule::DenyAll,
                AccessControllerRoleRule::DenyAll,
            ),
            (
                basic(ScryptoBasicRequirement::Require(ron(&a))),
                threshold(1, vec![nf(signer_a())]),
            ),
            (
                basic(ScryptoBasicRequirement::CountOf(
                    2,
                    vec![ron(&a), ron(&b)],
                )),
                threshold(2, vec![nf(signer_a()), nf(signer_b())]),
            ),
            (
                basic(ScryptoBasicRequirement::AnyOf(vec![ron(&a), ron(&b)])),
                threshold(1, vec![nf(signer_a()), nf(signer_b())]),
            ),
            (
                basic(ScryptoBasicRequirement::AllOf(vec![ron(&a), ron(&b)])),
                threshold(2, vec![nf(signer_a()), nf(signer_b())]),
            ),
            (
                basic(ScryptoBasicRequirement::AmountOf(
                    ScryptoDecimal192::ONE,
                    radix_common::constants::XRD,
                )),
                AccessControllerRoleRule::Complex,
            ),
            (
                ScryptoAccessRule::Protected(
                    ScryptoCompositeRequirement::AnyOf(vec![]),
                ),
                AccessControllerRoleRule::Complex,
            ),
        ];
        for (rule, expected) in cases {
            assert_eq!(
                AccessControllerRoleRule::from_scrypto(
                    rule.clone(),
                    network_id
                ),
                expected,
                "for {rule:?}"
            );
        }
    }

    // ==================
    // Default behaviour is unchanged
    // ==================

    #[test]
    fn without_policy_every_controller_call_is_rejected() {
        for (call, _) in supported_calls(external_ac()) {
            let sut = manifest(format!("{call}\n{}", spend()));
            let reserved = rejected_reserved_instructions(
                sut.validated_summary_for_pre_authorization(None),
            );
            assert_eq!(reserved, "AccessControllerMethod", "for `{call}`");
        }
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
        // that an AccessController is operated.
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
    fn policy_permits_every_supported_operation_alone() {
        // Recovery-family calls are sent alone: no account is touched.
        for (call, _) in supported_calls(external_ac()) {
            let sut = manifest(&call);
            assert_flagged_by_ret(&sut, &call);
            assert_eq!(
                sut.validated_summary_for_pre_authorization(Some(
                    &SUT::default()
                )),
                Ok(sut.summary().unwrap()),
                "for `{call}`"
            );
        }
    }

    #[test]
    fn policy_permits_several_external_access_controllers() {
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
    fn policy_permits_external_controller_when_other_controllers_are_in_profile(
    ) {
        let sut = multisig_spend(external_ac());
        let policy = SUT::new([other_external_ac()]);
        assert!(sut
            .validated_summary_for_pre_authorization(Some(&policy))
            .is_ok());
    }

    // ==================
    // (1) Only supported operations
    // ==================

    #[test]
    fn policy_rejects_every_unsupported_method() {
        // `withdraw_recovery_fee` puts XRD on the worktop; deposit it so the
        // manifest is valid and only the controller call is under test.
        let deposit = format!(
            r#"CALL_METHOD Address("{}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;"#,
            recipient()
        );
        for (call, _) in unsupported_calls(external_ac()) {
            let sut = manifest(format!("{call}\n{deposit}"));
            assert_flagged_by_ret(&sut, &call);
            let reserved = rejected_reserved_instructions(
                sut.validated_summary_for_pre_authorization(Some(
                    &SUT::default(),
                )),
            );
            assert_eq!(reserved, "AccessControllerMethod", "for `{call}`");
        }
    }

    #[test]
    fn policy_rejects_supported_call_combined_with_unsupported_call() {
        let ac = external_ac();
        let sut = manifest(format!(
            r#"{}
CALL_METHOD Address("{ac}") "initiate_badge_withdraw_attempt_as_primary";
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
    // (3) The wallet's own controllers are never exempt
    // ==================

    #[test]
    fn policy_rejects_every_operation_on_access_controller_of_profile_entity() {
        let policy = SUT::new([external_ac()]);
        for (call, _) in supported_calls(external_ac()) {
            let sut = manifest(&call);
            let reserved = rejected_reserved_instructions(
                sut.validated_summary_for_pre_authorization(Some(&policy)),
            );
            assert_eq!(reserved, "AccessControllerMethod", "for `{call}`");
        }
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
    // (4) No role-assignment changes
    // ==================

    #[test]
    fn policy_rejects_role_changes_with_any_supported_operation() {
        let account = multisig_account();
        let attacks = [
            format!(r#"SET_OWNER_ROLE Address("{account}") Enum<0u8>();"#),
            format!(r#"LOCK_OWNER_ROLE Address("{account}");"#),
            format!(
                r#"SET_ROLE Address("{account}") Enum<0u8>() "securify" Enum<0u8>();"#
            ),
        ];
        for (call, _) in supported_calls(external_ac()) {
            for attack in &attacks {
                let sut = manifest(format!("{call}\n{attack}"));
                let reserved = rejected_reserved_instructions(
                    sut.validated_summary_for_pre_authorization(Some(
                        &SUT::default(),
                    )),
                );
                assert_eq!(
                    reserved, "AccessControllerMethod",
                    "for `{call}` + `{attack}`"
                );
            }
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
