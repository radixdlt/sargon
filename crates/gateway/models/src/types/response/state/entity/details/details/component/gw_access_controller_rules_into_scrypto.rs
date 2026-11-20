//! Access controller rules as the Gateway returns them, converted to the
//! engine's: the rules a recovery proposes, and the current ones in the
//! access controller's role assignments.

use crate::prelude::*;

impl From<RecoveryProposal> for ScryptoRuleSet {
    fn from(
        RecoveryProposal {
            primary_role,
            recovery_role,
            confirmation_role,
            ..
        }: RecoveryProposal,
    ) -> Self {
        Self {
            primary_role: primary_role.into(),
            recovery_role: recovery_role.into(),
            confirmation_role: confirmation_role.into(),
        }
    }
}

impl From<AccessRule> for ScryptoAccessRule {
    fn from(value: AccessRule) -> Self {
        match value {
            AccessRule::AllowAll => Self::AllowAll,
            AccessRule::DenyAll => Self::DenyAll,
            AccessRule::Protected { access_rule } => {
                Self::Protected(access_rule.into())
            }
        }
    }
}

impl From<CompositeRequirement> for ScryptoCompositeRequirement {
    fn from(value: CompositeRequirement) -> Self {
        match value {
            CompositeRequirement::ProofRule { proof_rule } => {
                Self::BasicRequirement(proof_rule.into())
            }
            CompositeRequirement::AnyOf { access_rules } => Self::AnyOf(
                access_rules.into_iter().map(|r| r.into()).collect(),
            ),
            CompositeRequirement::AllOf { access_rules } => Self::AllOf(
                access_rules.into_iter().map(|r| r.into()).collect(),
            ),
        }
    }
}

impl From<BasicRequirement> for ScryptoBasicRequirement {
    fn from(value: BasicRequirement) -> Self {
        match value {
            BasicRequirement::Require { requirement } => {
                Self::Require(requirement.into())
            }
            BasicRequirement::AmountOf { amount, resource } => {
                Self::AmountOf(amount.into(), resource.into())
            }
            BasicRequirement::AllOf { list } => {
                let out = list.into_iter().map(|r| r.into()).collect();
                Self::AllOf(out)
            }
            BasicRequirement::AnyOf { list } => {
                let out = list.into_iter().map(|r| r.into()).collect();
                Self::AnyOf(out)
            }
            BasicRequirement::CountOf { count, list } => {
                let count_u8: u8 = count.try_into().expect(
                    "legacy BasicRequirement::CountOf count too large for u8",
                );
                let out = list.into_iter().map(|r| r.into()).collect();
                Self::CountOf(count_u8, out)
            }
        }
    }
}

impl From<Requirement> for ScryptoResourceOrNonFungible {
    fn from(value: Requirement) -> Self {
        match value {
            Requirement::Resource { resource } => {
                Self::Resource(resource.into())
            }
            Requirement::NonFungible { non_fungible } => {
                Self::NonFungible(non_fungible.into())
            }
        }
    }
}

impl From<NonFungible> for ScryptoNonFungibleGlobalId {
    fn from(value: NonFungible) -> Self {
        Self::new(value.resource_address.into(), value.local_id.into())
    }
}

impl From<NonFungibleLocalIdId> for ScryptoNonFungibleLocalId {
    fn from(v: NonFungibleLocalIdId) -> Self {
        Self::from_str(&v.simple_rep)
            .expect("Failed to parse NonFungibleLocalId from GW simple_rep")
    }
}

/// The current rules of an access controller, read from its role assignments.
impl TryFrom<ComponentEntityRoleAssignments> for ScryptoRuleSet {
    type Error = CommonError;

    fn try_from(assignments: ComponentEntityRoleAssignments) -> Result<Self> {
        let rule_of = |role: &str| -> Result<ScryptoAccessRule> {
            assignments
                .entries
                .iter()
                .find(|entry| {
                    entry.role_key.module == ObjectModuleId::Main
                        && entry.role_key.name == role
                })
                .and_then(|entry| entry.assignment.explicit_rule.clone())
                .map(ScryptoAccessRule::from)
                .ok_or_else(|| CommonError::Unknown {
                    error_message: format!(
                        "Access controller role '{role}' has no explicit rule"
                    ),
                })
        };
        Ok(Self {
            primary_role: rule_of("primary")?,
            recovery_role: rule_of("recovery")?,
            confirmation_role: rule_of("confirmation")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::fixture_gw_model;
    use radix_engine_interface::prelude::{require, require_n_of, AccessRule};

    /// The role assignments of a shared account's access controller on
    /// Stokenet, from the Gateway's details of it.
    fn assignments() -> ComponentEntityRoleAssignments {
        let details: serde_json::Value = serde_json::from_str(
            fixture_gw_model!("state/shared_account_access_controller_details"),
        )
        .unwrap();
        serde_json::from_value(
            details["items"][0]["details"]["role_assignments"].clone(),
        )
        .unwrap()
    }

    /// Primary and confirmation are one key; recovery is any 2 of that key,
    /// the wallet's key and a third.
    #[test]
    fn rules_are_read_from_each_role() {
        let key = |hex: &str| {
            ScryptoNonFungibleGlobalId::from(
                NonFungibleGlobalId::from_str(&format!(
                    "resource_tdx_2_1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxx3e2cpa:[{hex}]"
                ))
                .unwrap(),
            )
        };
        let other =
            key("b236b78acd067cc4c4c3f5fb686c7f4a765d6cb3cd2ddfd25310fec1a0");
        let wallet =
            key("fd1fdc334ff599f43da12459e733579c74a8797e92161a7e8847a8eea4");
        let third =
            key("76541ca712dcdccc5f176a50d7f90b3cf560124e69d5062974a1dd7111");

        assert_eq!(
            ScryptoRuleSet::try_from(assignments()).unwrap(),
            ScryptoRuleSet {
                primary_role: AccessRule::Protected(require(other.clone())),
                recovery_role: AccessRule::Protected(require_n_of(
                    2u8,
                    vec![wallet, other.clone(), third],
                )),
                confirmation_role: AccessRule::Protected(require(other)),
            }
        );
    }

    #[test]
    fn a_missing_role_is_an_error() {
        let mut assignments = assignments();
        assignments
            .entries
            .retain(|entry| entry.role_key.name != "confirmation");

        assert!(ScryptoRuleSet::try_from(assignments).is_err());
    }
}
