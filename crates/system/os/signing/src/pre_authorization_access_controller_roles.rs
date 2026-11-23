use crate::prelude::*;
use RoleKind::*;

/// The roles the wallet signs for, in a pre-authorization, on each access
/// controller it calls, read from its classification, or `None` when it isn't
/// an access controller class.
///
/// Only for pre-authorizations, signed together with others, e.g. the
/// members of a shared account: the wallet can't see their signatures, so it
/// signs for every role that may help. The wallet's own access controller
/// transactions are signed through their own flows, which know the roles the
/// user's factors can complete (see `SigningFlow`).
///
/// A pre-authorization's signatures all go into one auth zone, and each call
/// checks its roles against all of them. So, per access controller:
/// - a role a call can only be made with is signed for;
/// - a call that can be made with any of several roles, e.g. a quick
///   confirmation of the recovery role's proposal, by the primary or the
///   confirmation role, adds them all, as the wallet can't know which one
///   the others complete;
/// - unless it accepts a role another call on the same access controller
///   requires: that role must be completed anyway, and it passes this call
///   too.
///
/// Each access controller the class names is listed, those whose calls need
/// no role with none: the wallet then signs nothing for it. One missing isn't
/// described, and signing then takes every role.
pub(crate) fn pre_authorization_access_controller_roles(
    class: &DetailedManifestClass,
) -> Option<IndexMap<AccessControllerAddress, IndexSet<RoleKind>>> {
    let mut needs = IndexMap::<AccessControllerAddress, CallsNeed>::new();
    match class {
        // A change made at once: per access controller, a proposal and its
        // quick confirmation.
        // - `initiate_recovery_as_recovery` / `_as_primary`: the proposing
        //   role only.
        // - `quick_confirm_recovery_role_recovery_proposal`: primary or
        //   confirmation; `quick_confirm_primary_role_recovery_proposal`:
        //   recovery or confirmation. Neither is the proposing role, so both
        //   are signed for.
        // So every role, whichever role proposes.
        DetailedManifestClass::AccessControllerRecovery {
            recovery_proposals,
            ..
        } => {
            for proposal in recovery_proposals {
                let need = needs.entry(proposal.access_controller).or_default();
                need.requires(proposing_role(proposal.proposer));
                need.accepts_any_of(confirming_roles(proposal.proposer));
            }
        }
        // A change completed after a wait: per access controller, a
        // proposal by the recovery role, the only role whose proposals
        // have a timer, optionally with the primary role locked until
        // then.
        // - `initiate_recovery_as_recovery`: recovery.
        // - `lock_primary_role`: recovery.
        // So the recovery role only.
        DetailedManifestClass::AccessControllerTimedRecovery {
            timed_recoveries,
            ..
        } => {
            for recovery in timed_recoveries {
                needs
                    .entry(recovery.access_controller)
                    .or_default()
                    .requires(Recovery);
            }
        }
        // Completing a change proposed earlier, per access controller:
        // - quickly, by a second role: any role but the proposing one,
        //   as for a change made at once;
        // - after the wait, `timed_confirm_recovery`: public, so no
        //   role, and no signature. The dApp could submit it without
        //   the user; asked anyway, the wallet signs nothing.
        DetailedManifestClass::AccessControllerConfirmRecovery {
            confirmations,
            ..
        } => {
            for confirmation in confirmations {
                let need =
                    needs.entry(confirmation.access_controller).or_default();
                match confirmation.kind {
                    RecoveryConfirmationKind::Quick => need.accepts_any_of(
                        confirming_roles(confirmation.proposer),
                    ),
                    RecoveryConfirmationKind::Timed => {}
                }
            }
        }
        // Stopping or cancelling a pending change, per access controller,
        // each at most once:
        // - `stop_timed_recovery`: any role;
        // - `cancel_recovery_role_recovery_proposal`: recovery;
        //   `cancel_primary_role_recovery_proposal`: primary;
        // - `unlock_primary_role`: recovery.
        // Stopping alone signs for every role. With a cancel or an unlock,
        // the role they require stops the timer too, so it's the only one
        // signed for.
        DetailedManifestClass::AccessControllerCancelRecovery {
            cancellations,
            ..
        } => {
            for cancellation in cancellations {
                let need =
                    needs.entry(cancellation.access_controller).or_default();
                if cancellation.stopped_timed_recovery.is_some() {
                    need.accepts_any_of([Primary, Recovery, Confirmation]);
                }
                for proposer in &cancellation.cancelled_proposers {
                    need.requires(proposing_role(*proposer));
                }
                if cancellation.unlocks_primary_role {
                    need.requires(Recovery);
                }
            }
        }
        // Locking or unlocking the primary role, per access controller:
        // `lock_primary_role` and `unlock_primary_role` both need the
        // recovery role.
        DetailedManifestClass::AccessControllerPrimaryRoleLock {
            operations,
            ..
        } => {
            for operation in operations {
                needs
                    .entry(operation.access_controller)
                    .or_default()
                    .requires(Recovery);
            }
        }
        // Moving XRD into or out of access controllers' recovery fee
        // vaults:
        // - `contribute_recovery_fee`: public, so no role;
        // - `withdraw_recovery_fee`: primary.
        // Pre-authorizations may not be of this class, but the roles are
        // given all the same.
        DetailedManifestClass::AccessControllerRecoveryFee {
            contributions,
            withdrawals,
            ..
        } => {
            for contribution in contributions {
                needs.entry(contribution.access_controller).or_default();
            }
            for withdrawal in withdrawals {
                needs
                    .entry(withdrawal.access_controller)
                    .or_default()
                    .requires(Primary);
            }
        }
        // Not an access controller class: nothing to say which roles.
        _ => return None,
    }
    Some(
        needs
            .into_iter()
            .map(|(access_controller, need)| (access_controller, need.roles()))
            .collect(),
    )
}

/// What the calls on one access controller need.
#[derive(Default)]
struct CallsNeed {
    /// The roles calls can only be made with.
    requires: IndexSet<RoleKind>,
    /// For each call that can be made with any of several roles, those
    /// roles: the wallet signs for all of them.
    accepts_any_of: Vec<IndexSet<RoleKind>>,
}

impl CallsNeed {
    /// A call can only be made with `role`.
    fn requires(&mut self, role: RoleKind) {
        self.requires.insert(role);
    }

    /// A call can be made with any of `roles`.
    fn accepts_any_of(&mut self, roles: impl IntoIterator<Item = RoleKind>) {
        self.accepts_any_of.push(roles.into_iter().collect());
    }

    /// The roles to sign for: every required role, and every role a call
    /// accepts, unless it accepts a required one, which completes it too.
    fn roles(self) -> IndexSet<RoleKind> {
        let mut roles = self.requires.clone();
        for accepted in self.accepts_any_of {
            if accepted.is_disjoint(&self.requires) {
                roles.extend(accepted);
            }
        }
        roles
    }
}

/// The role a proposal is made with.
fn proposing_role(proposer: RecoveryProposer) -> RoleKind {
    match proposer {
        RecoveryProposer::Primary => Primary,
        RecoveryProposer::Recovery => Recovery,
    }
}

/// The roles that may quick confirm a proposal: any other than the one it was
/// made with.
fn confirming_roles(proposer: RecoveryProposer) -> [RoleKind; 2] {
    match proposer {
        RecoveryProposer::Primary => [Recovery, Confirmation],
        RecoveryProposer::Recovery => [Primary, Confirmation],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ac() -> AccessControllerAddress {
        AccessControllerAddress::sample_stokenet()
    }

    fn other_ac() -> AccessControllerAddress {
        AccessControllerAddress::sample_stokenet_other()
    }

    fn set<const N: usize>(roles: [RoleKind; N]) -> IndexSet<RoleKind> {
        roles.into_iter().collect()
    }

    /// The roles for `ac()`, which the class must list.
    fn roles(class: DetailedManifestClass) -> IndexSet<RoleKind> {
        pre_authorization_access_controller_roles(&class)
            .expect("an access controller class")
            .get(&ac())
            .cloned()
            .expect("the access controller is listed")
    }

    fn change_made_at_once(
        proposer: RecoveryProposer,
    ) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerRecovery {
            ac_addresses: vec![ac()],
            recovery_proposals: vec![AccessControllerRecoveryProposal {
                access_controller: ac(),
                proposer,
                ..AccessControllerRecoveryProposal::sample()
            }],
        }
    }

    fn timed_change(locks_primary_role: bool) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerTimedRecovery {
            ac_addresses: vec![ac()],
            timed_recoveries: vec![AccessControllerTimedRecoveryProposal {
                access_controller: ac(),
                locks_primary_role,
                ..AccessControllerTimedRecoveryProposal::sample()
            }],
        }
    }

    fn confirmation(
        proposer: RecoveryProposer,
        kind: RecoveryConfirmationKind,
    ) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerConfirmRecovery {
            ac_addresses: vec![ac()],
            confirmations: vec![AccessControllerRecoveryConfirmation {
                access_controller: ac(),
                proposer,
                kind,
                ..AccessControllerRecoveryConfirmation::sample()
            }],
        }
    }

    fn cancellation_on(
        access_controller: AccessControllerAddress,
        stops: bool,
        cancelled_proposers: Vec<RecoveryProposer>,
        unlocks_primary_role: bool,
    ) -> AccessControllerRecoveryCancellation {
        AccessControllerRecoveryCancellation {
            access_controller,
            stopped_timed_recovery: stops
                .then(AccessControllerRuleSetForDisplay::sample),
            cancelled_proposers,
            unlocks_primary_role,
            controlled_entity: None,
        }
    }

    fn cancellation(
        stops: bool,
        cancelled_proposers: Vec<RecoveryProposer>,
        unlocks_primary_role: bool,
    ) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerCancelRecovery {
            ac_addresses: vec![ac()],
            cancellations: vec![cancellation_on(
                ac(),
                stops,
                cancelled_proposers,
                unlocks_primary_role,
            )],
        }
    }

    fn primary_role_lock(
        operation: PrimaryRoleLockOperation,
    ) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerPrimaryRoleLock {
            ac_addresses: vec![ac()],
            operations: vec![AccessControllerPrimaryRoleLockOperation {
                access_controller: ac(),
                operation,
                controlled_entity: None,
            }],
        }
    }

    fn recovery_fee(contributes: bool) -> DetailedManifestClass {
        let transfer = AccessControllerXrdTransfer {
            access_controller: ac(),
            xrd_amount: Decimal192::one(),
            controlled_entity: None,
        };
        DetailedManifestClass::AccessControllerRecoveryFee {
            ac_addresses: vec![ac()],
            contributions: if contributes {
                vec![transfer.clone()]
            } else {
                vec![]
            },
            withdrawals: if contributes { vec![] } else { vec![transfer] },
            account_withdrawals: vec![],
            account_deposits: vec![],
        }
    }

    #[test]
    fn other_classes_name_no_roles() {
        assert_eq!(
            pre_authorization_access_controller_roles(
                &DetailedManifestClass::General
            ),
            None
        );
    }

    #[test]
    fn a_change_made_at_once_needs_the_proposing_role_and_both_that_may_confirm(
    ) {
        assert_eq!(
            roles(change_made_at_once(RecoveryProposer::Recovery)),
            set([Recovery, Primary, Confirmation])
        );
        assert_eq!(
            roles(change_made_at_once(RecoveryProposer::Primary)),
            set([Primary, Recovery, Confirmation])
        );
    }

    #[test]
    fn a_timed_change_needs_the_recovery_role_with_or_without_a_lock() {
        assert_eq!(roles(timed_change(false)), set([Recovery]));
        assert_eq!(roles(timed_change(true)), set([Recovery]));
    }

    #[test]
    fn a_quick_confirmation_needs_both_roles_other_than_the_proposing_one() {
        assert_eq!(
            roles(confirmation(
                RecoveryProposer::Recovery,
                RecoveryConfirmationKind::Quick
            )),
            set([Primary, Confirmation])
        );
        assert_eq!(
            roles(confirmation(
                RecoveryProposer::Primary,
                RecoveryConfirmationKind::Quick
            )),
            set([Recovery, Confirmation])
        );
    }

    #[test]
    fn a_confirmation_after_the_wait_lists_its_access_controller_with_no_role()
    {
        assert_eq!(
            roles(confirmation(
                RecoveryProposer::Recovery,
                RecoveryConfirmationKind::Timed
            )),
            IndexSet::new()
        );
    }

    #[test]
    fn stopping_alone_needs_every_role() {
        assert_eq!(
            roles(cancellation(true, vec![], false)),
            set([Primary, Recovery, Confirmation])
        );
    }

    #[test]
    fn cancelling_needs_the_proposing_role() {
        assert_eq!(
            roles(cancellation(false, vec![RecoveryProposer::Recovery], false)),
            set([Recovery])
        );
        assert_eq!(
            roles(cancellation(false, vec![RecoveryProposer::Primary], false)),
            set([Primary])
        );
    }

    #[test]
    fn a_role_another_call_requires_covers_stopping() {
        // Stopping and cancelling the recovery role's proposal.
        assert_eq!(
            roles(cancellation(true, vec![RecoveryProposer::Recovery], false)),
            set([Recovery])
        );
        // Stopping and cancelling the primary role's proposal.
        assert_eq!(
            roles(cancellation(true, vec![RecoveryProposer::Primary], false)),
            set([Primary])
        );
        // Stopping and unlocking the primary role.
        assert_eq!(roles(cancellation(true, vec![], true)), set([Recovery]));
    }

    #[test]
    fn a_role_required_on_one_access_controller_doesnt_cover_another() {
        let sut = pre_authorization_access_controller_roles(
            &DetailedManifestClass::AccessControllerCancelRecovery {
                ac_addresses: vec![ac(), other_ac()],
                cancellations: vec![
                    cancellation_on(
                        ac(),
                        true,
                        vec![RecoveryProposer::Recovery],
                        false,
                    ),
                    cancellation_on(other_ac(), true, vec![], false),
                ],
            },
        )
        .unwrap();
        assert_eq!(sut[&ac()], set([Recovery]));
        assert_eq!(sut[&other_ac()], set([Primary, Recovery, Confirmation]));
    }

    #[test]
    fn locking_and_unlocking_need_the_recovery_role() {
        assert_eq!(
            roles(primary_role_lock(PrimaryRoleLockOperation::Lock)),
            set([Recovery])
        );
        assert_eq!(
            roles(primary_role_lock(PrimaryRoleLockOperation::Unlock)),
            set([Recovery])
        );
    }

    #[test]
    fn recovery_fees_need_the_primary_role_only_to_withdraw() {
        assert_eq!(roles(recovery_fee(true)), IndexSet::new());
        assert_eq!(roles(recovery_fee(false)), set([Primary]));
    }
}
