use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
#[serde(tag = "type")]
pub enum StateEntityDetailsResponseItemDetails {
    FungibleResource(StateEntityDetailsResponseFungibleResourceDetails),
    NonFungibleResource(StateEntityDetailsResponseNonFungibleResourceDetails),
    FungibleVault,
    NonFungibleVault,
    Package(StateEntityDetailsResponsePackageDetails),
    Component(StateEntityDetailsResponseComponentDetails),
}

impl StateEntityDetailsResponseItemDetails {
    /// Returns true if the entity can be transferred.
    ///
    /// To determine whether an entity can be transferred, we need to check its assignments
    /// for the ("Main", "depositor") & ("Main", "withdrawer") roles.
    /// Once we have them, we first check if the assignments are `Explicit` or `Owner`.
    /// - If they are `Explicit`, the entity will be transferable if both the depositor & withdrawer
    ///     have `AllowAll` rules.
    /// - If they are `Owner`, the entity will be transferable if the owner has `AllowAll` rule.
    pub fn can_be_transferred(&self) -> bool {
        let Some(role_assignments) = self.role_assignments() else {
            return false;
        };
        let Some(depositor) = role_assignments
            .entries
            .clone()
            .into_iter()
            .find(|entry| entry.role_key == RoleKey::main_depositor())
        else {
            return false;
        };

        if depositor.assignment.resolution == RoleAssignmentResolution::Owner {
            // No need to check for withdrawer, as its `resolution` will be the same.
            return role_assignments.owner.rule == ExplicitRule::AllowAll;
        }

        let Some(withdrawer) = role_assignments
            .entries
            .into_iter()
            .find(|entry| entry.role_key == RoleKey::main_withdrawer())
        else {
            return false;
        };

        let allows_all =
            |role_assignment: &ComponentEntityRoleAssignmentEntry| {
                role_assignment.assignment.explicit_rule
                    == Some(ExplicitRule::AllowAll)
            };
        let depositor_allows_all = allows_all(&depositor);
        let withdrawer_allows_all = allows_all(&withdrawer);
        // Both depositor and withdrawer must allow
        depositor_allows_all && withdrawer_allows_all
    }

    fn role_assignments(&self) -> Option<ComponentEntityRoleAssignments> {
        match self {
            Self::FungibleResource(details) => {
                Some(details.role_assignments.clone())
            }
            Self::NonFungibleResource(details) => {
                Some(details.role_assignments.clone())
            }
            Self::FungibleVault => None,
            Self::NonFungibleVault => None,
            Self::Package(details) => details.role_assignments.clone(),
            Self::Component(details) => details.role_assignments.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_gw_model;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsResponseItemDetails;

    #[test]
    #[allow(non_snake_case)]
    fn can_be_transferred() {
        type Assignments = ComponentEntityRoleAssignments;

        // Define a few resolution combinations
        let explicit = ComponentEntityRoleAssignmentOwner::sample_protected();
        let owner_allow_all =
            ComponentEntityRoleAssignmentOwner::sample_allow_all();
        let owner_deny_all =
            ComponentEntityRoleAssignmentOwner::sample_deny_all();

        // Define a few assignments combinations that cannot be transferred
        // Naming will be like this: <resolution>__<assignments>
        let explicit__empty = Assignments::new(explicit.clone(), []);
        let explicit__only_depositor = Assignments::new(
            explicit.clone(),
            [
                ComponentEntityRoleAssignmentEntry::sample_depositor_explicit_allow_all(
                ),
            ],
        );
        let explicit__depositor_allow_withdrawer_deny = Assignments::new(explicit.clone(), [
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::main_depositor(),
                ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_allow_all(
                ),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::main_withdrawer(),
                ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_deny_all(),
            ),
        ]);
        let explicit__depositor_deny_withdrawer_allow = Assignments::new(explicit.clone(), [
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::main_depositor(),
                ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_deny_all(),
            ),
            ComponentEntityRoleAssignmentEntry::new(
                RoleKey::main_withdrawer(),
                ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_allow_all(
                ),
            ),
        ]);
        let owner__deny_all =
            Assignments::new(
                owner_deny_all.clone(),
                [
                    ComponentEntityRoleAssignmentEntry::sample_depositor_owner_allow_all(),
                    ComponentEntityRoleAssignmentEntry::sample_withdrawer_owner_allow_all(),
        ],
            );

        // Define a few assignments combinations that can be transferred
        let explicit__allow_all = Assignments::sample_allow_all();
        let owner__allow_all =
            Assignments::new(owner_allow_all.clone(), [
                ComponentEntityRoleAssignmentEntry::sample_depositor_owner_allow_all(),
                ComponentEntityRoleAssignmentEntry::sample_withdrawer_owner_allow_all(),
            ]);

        // Define the two outcomes
        let cannot_be_transferred = vec![
            explicit__empty,
            explicit__only_depositor,
            explicit__depositor_allow_withdrawer_deny,
            explicit__depositor_deny_withdrawer_allow,
            owner__deny_all,
        ];
        let can_be_transferred = vec![explicit__allow_all, owner__allow_all];

        // Test the combinations for FungibleResource
        for assignments in cannot_be_transferred.clone() {
            let sut = SUT::FungibleResource(
                StateEntityDetailsResponseFungibleResourceDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(!sut.can_be_transferred());
        }
        for assignments in can_be_transferred.clone() {
            let sut = SUT::FungibleResource(
                StateEntityDetailsResponseFungibleResourceDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(sut.can_be_transferred());
        }

        // Test the combinations for NonFungibleResource
        for assignments in cannot_be_transferred.clone() {
            let sut = SUT::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(!sut.can_be_transferred());
        }
        for assignments in can_be_transferred.clone() {
            let sut = SUT::NonFungibleResource(
                StateEntityDetailsResponseNonFungibleResourceDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(sut.can_be_transferred());
        }

        // Test the combinations for Package
        for assignments in cannot_be_transferred.clone() {
            let sut =
                SUT::Package(StateEntityDetailsResponsePackageDetails::new(
                    assignments.clone(),
                ));
            assert!(!sut.can_be_transferred());
        }
        for assignments in can_be_transferred.clone() {
            let sut =
                SUT::Package(StateEntityDetailsResponsePackageDetails::new(
                    assignments.clone(),
                ));
            assert!(sut.can_be_transferred());
        }

        // Test the combinations for Component
        for assignments in cannot_be_transferred.clone() {
            let sut = SUT::Component(
                StateEntityDetailsResponseComponentDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(!sut.can_be_transferred());
        }
        for assignments in can_be_transferred.clone() {
            let sut = SUT::Component(
                StateEntityDetailsResponseComponentDetails::new(
                    assignments.clone(),
                ),
            );
            assert!(sut.can_be_transferred());
        }

        // Test for FungibleVault & NonFungibleVault
        let sut = SUT::FungibleVault;
        assert!(!sut.can_be_transferred());
        let sut = SUT::NonFungibleVault;
        assert!(!sut.can_be_transferred());
    }

    #[test]
    fn json() {
        // Note: we aren't using `assert_eq_after_json_roundtrip` to verify the roundtrip because there
        // are multiple fields that we aren't parsing, so we can't compare the entire struct.

        // Fungible Resource (XRD)
        let result = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details_details__fungible_resource"
        ))
        .unwrap()
        .0;

        assert!(matches!(result, SUT::FungibleResource(_)));

        // Non-Fungible Resource (XRD)
        let result = fixture_and_json::<SUT>(fixture_gw_model!(
            "state/response_entity_details_details__non_fungible_resource"
        ))
        .unwrap()
        .0;

        assert!(matches!(result, SUT::NonFungibleResource(_)));
    }

    #[test]
    fn component_state_is_stored_as_string() {
        use serde_json::Value;

        let json = r#"{
            "type": "Component",
            "state": {
                "xrd_fee_vault": null,
                "controlled_vault": {
                    "is_global": false,
                    "entity_type": "InternalNonFungibleVault",
                    "entity_address": "internal_vault_tdx_2_1nrs3a0qw5qfsx83hvkn8h6l6pfsc703gr8k5wz24gup97zxjsct0t7"
                },
                "is_primary_role_locked": false,
                "timed_recovery_delay_minutes": 20160,
                "primary_role_recovery_attempt": null,
                "recovery_role_recovery_attempt": null,
                "recovery_badge_resource_address": "resource_tdx_2_1n2tf2eyhphq7r9q2kcejnrvnevl2v2hl97rtddjzv5kfs8jvcsx6as",
                "has_primary_role_badge_withdraw_attempt": false,
                "has_recovery_role_badge_withdraw_attempt": false
            }
        }"#;

        let parsed: SUT =
            serde_json::from_str(json).expect("valid component json");
        let StateEntityDetailsResponseItemDetails::Component(details) = parsed
        else {
            panic!("expected Component variant");
        };

        // state should be stored as a raw JSON string
        let state_raw = details.state.as_ref().expect("state should be Some");

        let ac_state: AccessControllerFieldStateValue =
            serde_json::from_str(state_raw).unwrap();
        print!("{:?}", ac_state);
    }
}
