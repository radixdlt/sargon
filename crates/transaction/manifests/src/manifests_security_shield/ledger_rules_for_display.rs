//! Real access controller rules for display: as the ledger stores them for a
//! shield Sargon applies, and as the Gateway returns them.

use crate::prelude::*;
use prelude::fixture_gw_model;
use profile_supporting_types::UnsecurifiedAccount;
use scrypto_test::prelude::LedgerSimulatorBuilder;

fn is_open_or_locked(requirement: &AccessRequirementForDisplay) -> bool {
    matches!(
        requirement,
        AccessRequirementForDisplay::Anyone
            | AccessRequirementForDisplay::NoOne
    )
}

/// A shield applied with Sargon's manifests, read back from the ledger. Its
/// recovery and confirmation roles only have override factors, which Sargon
/// writes as `AnyOf[CountOf(0, []), AnyOf(overrides)]`. The engine passes a
/// zero count with no proof, so they are shown as `Anyone`. This changes once
/// the shield serializer no longer writes a zero count.
#[test]
fn shield_applied_by_sargon_as_stored_on_ledger() {
    let mut ledger =
        LedgerSimulatorBuilder::new().without_kernel_trace().build();
    let shield = SecurityStructureOfFactorInstances::sample_sim();
    let account = ledger.securify_account(
        UnsecurifiedAccount::sample_sim_account(),
        shield.clone(),
    );

    let on_ledger = ledger
        .read_access_controller_rule_set(account.access_controller_address());
    assert_eq!(
        on_ledger,
        ScryptoRuleSet::from(shield.matrix_of_factors.clone())
    );

    let shown =
        AccessControllerRuleSet::from((on_ledger, NetworkID::Simulator))
            .for_display();
    assert!(!is_open_or_locked(&shown.primary_role));
    assert_eq!(shown.recovery_role, AccessRequirementForDisplay::Anyone);
    assert_eq!(shown.confirmation_role, AccessRequirementForDisplay::Anyone);
}

/// A recovery proposal on Stokenet as the Gateway returns it: the same shield
/// shape, through the Gateway's JSON. It is shown the same way as the rules
/// read from the ledger above.
#[test]
fn shield_proposal_as_returned_by_gateway() {
    let attempt = fixture_and_json::<RecoveryRoleRecoveryAttempt>(
        fixture_gw_model!("state/ac_state_details_recovery_attempt"),
    )
    .unwrap()
    .0;

    let shown = AccessControllerRuleSet::from((
        ScryptoRuleSet::from(attempt.recovery_proposal),
        NetworkID::Stokenet,
    ))
    .for_display();

    // AnyOf[CountOf(1, [key]), AnyOf([])] is that one key.
    assert!(matches!(
        shown.primary_role,
        AccessRequirementForDisplay::Badge {
            badge: BadgeForDisplay::UnknownKey { .. }
        }
    ));
    assert_eq!(shown.recovery_role, AccessRequirementForDisplay::Anyone);
    assert_eq!(shown.confirmation_role, AccessRequirementForDisplay::Anyone);
}
