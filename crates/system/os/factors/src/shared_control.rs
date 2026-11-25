use crate::prelude::*;

/// The access controller operations a pre-authorization may make: changing,
/// confirming or cancelling an access controller's rules, and locking or
/// unlocking its primary role. Each class shows everything its manifest does,
/// and [`shared_control_refusal`] makes sure the user alone passes none of
/// the roles needed, so others sharing control must sign too.
///
/// Moving XRD out of an access controller's recovery fee vault isn't allowed:
/// it can deposit into any account, which needs no authorization of its own.
pub fn is_allowed_in_pre_authorizations(class: &DetailedManifestClass) -> bool {
    matches!(
        class,
        DetailedManifestClass::AccessControllerRecovery { .. }
            | DetailedManifestClass::AccessControllerTimedRecovery { .. }
            | DetailedManifestClass::AccessControllerConfirmRecovery { .. }
            | DetailedManifestClass::AccessControllerCancelRecovery { .. }
            | DetailedManifestClass::AccessControllerPrimaryRoleLock { .. }
    )
}

/// The rules `class` makes the rules of access controllers, or lets become
/// them, as the review shows them. Not those of a timed recovery it stops:
/// stopping enacts nothing, and it must stay possible whatever was proposed.
fn rules_to_enact(
    class: &DetailedManifestClass,
) -> Vec<&AccessControllerRuleSetForDisplay> {
    match class {
        DetailedManifestClass::AccessControllerRecovery {
            recovery_proposals,
            ..
        } => recovery_proposals
            .iter()
            .map(|proposal| &proposal.proposed_rules)
            .collect(),
        DetailedManifestClass::AccessControllerTimedRecovery {
            timed_recoveries,
            ..
        } => timed_recoveries
            .iter()
            .map(|recovery| &recovery.proposed_rules)
            .collect(),
        DetailedManifestClass::AccessControllerConfirmRecovery {
            confirmations,
            ..
        } => confirmations
            .iter()
            .map(|confirmation| &confirmation.proposed_rules)
            .collect(),
        DetailedManifestClass::SecurifiedEntityCreation {
            created_entities,
        } => created_entities
            .iter()
            .map(|created| &created.rules)
            .collect(),
        _ => Vec::new(),
    }
}

/// Whether `class` enacts rules that count a holder more than once, e.g.
/// "any 2 of these 3" with one key listed twice, which that key passes
/// alone: the review would show control as shared when it isn't.
pub fn enacts_rules_with_overlapping_entries(
    class: &DetailedManifestClass,
) -> bool {
    rules_to_enact(class)
        .into_iter()
        .any(AccessControllerRuleSetForDisplay::has_overlapping_entries)
}

/// Whether `auth_zone` alone passes any role of any of `access_controllers`,
/// with their current `rule_sets`. An access controller without known rules
/// counts as passed.
pub fn can_pass_any_role_alone(
    access_controllers: &IndexSet<AccessControllerAddress>,
    rule_sets: &IndexMap<AccessControllerAddress, AccessControllerRuleSet>,
    auth_zone: &AuthZone,
) -> bool {
    access_controllers.iter().any(|access_controller| {
        rule_sets.get(access_controller).is_none_or(|rule_set| {
            [
                &rule_set.primary_role,
                &rule_set.recovery_role,
                &rule_set.confirmation_role,
            ]
            .into_iter()
            .any(|role| role.is_satisfied_by(auth_zone))
        })
    })
}

/// What the user can prove alone in a pre-authorization: the signatures of
/// the keys it is signed with, see [`OsMfaSigningKeys`].
pub fn auth_zone_of(keys: Vec<MfaSigningKey>) -> AuthZone {
    AuthZone {
        implicit_non_fungibles: keys.into_iter().map(|key| key.badge).collect(),
        proofs: Vec::new(),
    }
}

/// Why a pre-authorization making access controller calls, of `class`, with
/// the static `summary`, must be refused, or `None` if it may be signed.
///
/// Access controller methods are reserved: a dApp may call them in a
/// pre-authorization only on an access controller the user controls together
/// with others, never one they control alone. So the calls are allowed only
/// when:
/// * the manifest is classified as an access controller operation allowed in
///   pre-authorizations, see [`is_allowed_in_pre_authorizations`], so it
///   makes no other calls;
/// * the rules it enacts name each holder once per group, see
///   [`enacts_rules_with_overlapping_entries`], so they read as what they
///   are;
/// * no account or persona needs to authorize it, so `keys` are the only
///   authority the user adds;
/// * each access controller called holds the owner badge of exactly one
///   account or persona, the one the review names;
/// * `keys` alone pass no role of any access controller called. Whether its
///   entity is in the user's Profile doesn't matter: a shared account can be
///   there too. Their current rules come from `access_controllers_on_ledger`;
///   one missing counts as passed.
///
/// Both the review and signing check this, signing with the rules read again
/// from the ledger.
pub fn shared_control_refusal(
    class: Option<&DetailedManifestClass>,
    summary: &ManifestSummary,
    access_controllers_on_ledger: &IndexMap<
        AccessControllerAddress,
        AccessControllerStateDetails,
    >,
    keys: Vec<MfaSigningKey>,
    network_id: NetworkID,
) -> Option<&'static str> {
    let Some(class) = class.filter(|class| is_allowed_in_pre_authorizations(class))
    else {
        return Some(
            "not classified as an access controller operation allowed in pre-authorizations",
        );
    };
    if enacts_rules_with_overlapping_entries(class) {
        return Some("the rules to enact count a holder more than once");
    }
    if !summary.addresses_of_accounts_requiring_auth.is_empty()
        || !summary.addresses_of_personas_requiring_auth.is_empty()
    {
        return Some("an account or persona needs to authorize it");
    }
    // Every access controller the calls name, whether the summary or the
    // class lists it.
    let access_controllers = summary
        .access_controllers_called
        .iter()
        .chain(class.access_controllers().unwrap_or_default())
        .copied()
        .collect::<IndexSet<_>>();
    if access_controllers.is_empty() {
        return Some("no access controller is called");
    }
    let controls_one_entity = |access_controller: &AccessControllerAddress| {
        access_controllers_on_ledger
            .get(access_controller)
            .is_some_and(|details| details.controlled_entities.len() == 1)
    };
    if !access_controllers.iter().all(controls_one_entity) {
        return Some(
            "an access controller called doesn't control exactly one account or persona",
        );
    }
    // An access controller whose rules weren't read is left out, which
    // counts as the user passing its roles alone.
    let rule_sets = access_controllers_on_ledger
        .values()
        .filter_map(|details| {
            let rule_set = details.rule_set().ok()?;
            Some((details.address, (rule_set, network_id).into()))
        })
        .collect();
    if can_pass_any_role_alone(
        &access_controllers,
        &rule_sets,
        &auth_zone_of(keys),
    ) {
        return Some("the user's signing keys alone pass a role");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn summary_calling(
        access_controllers: Vec<AccessControllerAddress>,
    ) -> ManifestSummary {
        ManifestSummary {
            access_controllers_called: access_controllers,
            ..ManifestSummary::new(
                HashMap::<AccountAddress, Vec<AccountWithdraw>>::new(),
                HashMap::<AccountAddress, AccountDeposits>::new(),
                [],
                [],
                [],
                [],
                [],
                [],
                [ReservedInstruction::AccessControllerMethod],
                [],
            )
        }
    }

    fn lock(access_controller: AccessControllerAddress) -> DetailedManifestClass {
        DetailedManifestClass::AccessControllerPrimaryRoleLock {
            ac_addresses: vec![access_controller],
            operations: vec![AccessControllerPrimaryRoleLockOperation {
                access_controller,
                ..AccessControllerPrimaryRoleLockOperation::sample()
            }],
        }
    }

    fn on_ledger(
        access_controller: AccessControllerAddress,
        controlled_entities: Vec<AddressOfAccountOrPersona>,
    ) -> IndexMap<AccessControllerAddress, AccessControllerStateDetails> {
        IndexMap::from([(
            access_controller,
            AccessControllerStateDetails {
                address: access_controller,
                controlled_entities,
                ..AccessControllerStateDetails::sample()
            },
        )])
    }

    /// Calls the classification lists but the summary doesn't are checked
    /// too, and a manifest calling none is refused.
    #[test]
    fn every_access_controller_named_is_checked() {
        let access_controller = AccessControllerAddress::sample();
        assert_eq!(
            shared_control_refusal(
                Some(&DetailedManifestClass::AccessControllerPrimaryRoleLock {
                    ac_addresses: Vec::new(),
                    operations: Vec::new(),
                }),
                &summary_calling(Vec::new()),
                &IndexMap::new(),
                Vec::new(),
                NetworkID::Mainnet,
            ),
            Some("no access controller is called")
        );
        // Known only from the classification, and not read from the ledger.
        assert!(shared_control_refusal(
            Some(&lock(access_controller)),
            &summary_calling(Vec::new()),
            &IndexMap::new(),
            Vec::new(),
            NetworkID::Mainnet,
        )
        .is_some());
    }

    /// The review names one entity per access controller; one holding the
    /// owner badges of none or several governs what it doesn't name.
    #[test]
    fn an_access_controller_must_control_exactly_one_entity() {
        let access_controller = AccessControllerAddress::sample();
        for controlled_entities in [
            Vec::new(),
            vec![
                AddressOfAccountOrPersona::sample(),
                AddressOfAccountOrPersona::sample_other(),
            ],
        ] {
            assert_eq!(
                shared_control_refusal(
                    Some(&lock(access_controller)),
                    &summary_calling(vec![access_controller]),
                    &on_ledger(access_controller, controlled_entities),
                    Vec::new(),
                    NetworkID::Mainnet,
                ),
                Some(
                    "an access controller called doesn't control exactly one account or persona"
                )
            );
        }
        // One, but its rules can't be read: counts as passed alone.
        assert_eq!(
            shared_control_refusal(
                Some(&lock(access_controller)),
                &summary_calling(vec![access_controller]),
                &on_ledger(
                    access_controller,
                    vec![AddressOfAccountOrPersona::sample()]
                ),
                Vec::new(),
                NetworkID::Mainnet,
            ),
            Some("the user's signing keys alone pass a role")
        );
    }

    /// A dApp creating a shared account must name each holder once too.
    #[test]
    fn creating_entities_with_rules_counting_a_key_twice_overlaps() {
        let key = |badge: NonFungibleGlobalId| BadgeForDisplay::UnknownKey {
            badge,
        };
        let counted_twice = AccessRequirementForDisplay::CountOf {
            count: 2,
            badges: vec![
                key(NonFungibleGlobalId::sample()),
                key(NonFungibleGlobalId::sample()),
                key(NonFungibleGlobalId::sample_other()),
            ],
        };
        let rules = AccessControllerRuleSetForDisplay {
            primary_role: counted_twice.clone(),
            recovery_role: counted_twice.clone(),
            confirmation_role: counted_twice,
        };
        let class = DetailedManifestClass::SecurifiedEntityCreation {
            created_entities: vec![CreatedSecurifiedEntity {
                rules,
                ..CreatedSecurifiedEntity::sample()
            }],
        };
        assert!(enacts_rules_with_overlapping_entries(&class));
    }
}
