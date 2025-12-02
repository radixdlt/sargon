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
