use crate::prelude::*;
use radix_common::constants::{
    ED25519_SIGNATURE_RESOURCE, GLOBAL_CALLER_RESOURCE,
    PACKAGE_OF_DIRECT_CALLER_RESOURCE, SECP256K1_SIGNATURE_RESOURCE,
    SYSTEM_EXECUTION_RESOURCE,
};
use radix_engine_interface::blueprints::resource::{
    BasicRequirement as ScryptoBasicRequirement,
    CompositeRequirement as ScryptoCompositeRequirement,
    ResourceOrNonFungible as ScryptoResourceOrNonFungible,
};

/// The withdraw rules of the transferred resources that not everybody may withdraw, as Scrypto
/// access rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WithdrawRules {
    rules: Vec<(ResourceAddress, ScryptoAccessRule)>,
}

impl WithdrawRules {
    /// From the withdraw rule (the `withdrawer` role) of each transferred resource, as read from
    /// the Gateway. The resources anybody may withdraw are left out. Fails if a rule cannot be
    /// converted to a Scrypto access rule (an invalid non-fungible id, a count above 255).
    pub fn new(
        rules: impl IntoIterator<Item = (ResourceAddress, AccessRule)>,
    ) -> Result<Self> {
        let rules = rules
            .into_iter()
            .filter(|(_, rule)| *rule != AccessRule::AllowAll)
            .map(|(resource, rule)| {
                scrypto_access_rule(rule).map(|rule| (resource, rule))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { rules })
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = &(ResourceAddress, ScryptoAccessRule)> {
        self.rules.iter()
    }

    /// The badges to look for in the account to know whether it can satisfy the rules.
    pub fn badges_to_look_up(&self) -> BadgesToLookUp {
        let mut look_up = BadgesToLookUp::default();
        for (resource, rule) in self.iter() {
            let network_id = resource.network_id();
            for_each_requirement(rule, &mut |requirement| match requirement {
                RuleLeaf::Require(ScryptoResourceOrNonFungible::Resource(
                    badge,
                )) if !is_implicit(badge) => {
                    look_up.insert_resource((*badge, network_id).into(), 1)
                }
                RuleLeaf::Require(
                    ScryptoResourceOrNonFungible::NonFungible(id),
                ) if !is_implicit(&id.resource_address()) => {
                    look_up.insert_non_fungible((id.clone(), network_id).into())
                }
                RuleLeaf::AmountOf(amount, badge) if !is_implicit(badge) => {
                    // The engine wants a proof of the resource even for `AmountOf(0)`.
                    look_up.insert_resource(
                        (*badge, network_id).into(),
                        ids_needed_for(amount).max(1),
                    )
                }
                _ => {}
            });
        }
        look_up
    }
}

/// The badges `rule` asks for, i.e. the resources an account can hold, in the order they appear.
pub(crate) fn badges_of(
    rule: &ScryptoAccessRule,
    network_id: NetworkID,
) -> Vec<ResourceAddress> {
    let mut badges = IndexSet::<ResourceAddress>::new();
    for_each_requirement(rule, &mut |requirement| {
        let badge = match requirement {
            RuleLeaf::Require(ScryptoResourceOrNonFungible::Resource(
                badge,
            )) => *badge,
            RuleLeaf::Require(ScryptoResourceOrNonFungible::NonFungible(
                id,
            )) => id.resource_address(),
            RuleLeaf::AmountOf(_, badge) => *badge,
        };
        if !is_implicit(&badge) {
            badges.insert((badge, network_id).into());
        }
    });
    badges.into_iter().collect()
}

/// The resources no account holds: the engine puts their proofs in the auth zone itself, e.g.
/// one per signature of the transaction, or one for the caller of a method.
pub(crate) fn is_implicit(resource: &ScryptoResourceAddress) -> bool {
    IMPLICIT_RESOURCES.contains(resource)
}

const IMPLICIT_RESOURCES: [ScryptoResourceAddress; 5] = [
    SECP256K1_SIGNATURE_RESOURCE,
    ED25519_SIGNATURE_RESOURCE,
    GLOBAL_CALLER_RESOURCE,
    PACKAGE_OF_DIRECT_CALLER_RESOURCE,
    SYSTEM_EXECUTION_RESOURCE,
];

/// How many non-fungibles prove an amount of `amount`.
pub(crate) fn ids_needed_for(amount: ScryptoDecimal192) -> u64 {
    if !amount.is_positive() {
        return 0;
    }
    amount
        .checked_ceiling()
        .and_then(|ceiling| ceiling.to_string().parse().ok())
        .unwrap_or(u64::MAX)
}

/// The Gateway rule as a Scrypto access rule. Unlike `From<AccessRule> for ScryptoAccessRule`,
/// this does not panic on data it cannot convert: the rules come from the ledger.
fn scrypto_access_rule(rule: AccessRule) -> Result<ScryptoAccessRule> {
    Ok(match rule {
        AccessRule::AllowAll => ScryptoAccessRule::AllowAll,
        AccessRule::DenyAll => ScryptoAccessRule::DenyAll,
        AccessRule::Protected { access_rule } => {
            ScryptoAccessRule::Protected(scrypto_composite(access_rule)?)
        }
    })
}

fn scrypto_composite(
    requirement: CompositeRequirement,
) -> Result<ScryptoCompositeRequirement> {
    Ok(match requirement {
        CompositeRequirement::ProofRule { proof_rule } => {
            ScryptoCompositeRequirement::BasicRequirement(scrypto_basic(
                proof_rule,
            )?)
        }
        CompositeRequirement::AnyOf { access_rules } => {
            ScryptoCompositeRequirement::AnyOf(
                access_rules
                    .into_iter()
                    .map(scrypto_composite)
                    .collect::<Result<_>>()?,
            )
        }
        CompositeRequirement::AllOf { access_rules } => {
            ScryptoCompositeRequirement::AllOf(
                access_rules
                    .into_iter()
                    .map(scrypto_composite)
                    .collect::<Result<_>>()?,
            )
        }
    })
}

fn scrypto_basic(
    requirement: BasicRequirement,
) -> Result<ScryptoBasicRequirement> {
    let list = |list: Vec<Requirement>| {
        list.into_iter()
            .map(scrypto_resource_or_non_fungible)
            .collect::<Result<Vec<_>>>()
    };
    Ok(match requirement {
        BasicRequirement::Require { requirement } => {
            ScryptoBasicRequirement::Require(scrypto_resource_or_non_fungible(
                requirement,
            )?)
        }
        BasicRequirement::AmountOf { amount, resource } => {
            ScryptoBasicRequirement::AmountOf(amount.into(), resource.into())
        }
        BasicRequirement::AllOf { list: l } => {
            ScryptoBasicRequirement::AllOf(list(l)?)
        }
        BasicRequirement::AnyOf { list: l } => {
            ScryptoBasicRequirement::AnyOf(list(l)?)
        }
        BasicRequirement::CountOf { count, list: l } => {
            let count =
                u8::try_from(count).map_err(|_| CommonError::Unknown {
                    error_message: format!(
                        "CountOf count {count} of a withdraw rule is above 255"
                    ),
                })?;
            ScryptoBasicRequirement::CountOf(count, list(l)?)
        }
    })
}

fn scrypto_resource_or_non_fungible(
    requirement: Requirement,
) -> Result<ScryptoResourceOrNonFungible> {
    Ok(match requirement {
        Requirement::Resource { resource } => {
            ScryptoResourceOrNonFungible::Resource(resource.into())
        }
        Requirement::NonFungible { non_fungible } => {
            let local_id = ScryptoNonFungibleLocalId::from_str(
                &non_fungible.local_id.simple_rep,
            )
            .map_err(|_| CommonError::InvalidNonFungibleLocalIDString)?;
            ScryptoResourceOrNonFungible::NonFungible(
                ScryptoNonFungibleGlobalId::new(
                    non_fungible.resource_address.into(),
                    local_id,
                ),
            )
        }
    })
}

/// A leaf of an access rule: a single requirement.
enum RuleLeaf<'a> {
    Require(&'a ScryptoResourceOrNonFungible),
    AmountOf(ScryptoDecimal192, &'a ScryptoResourceAddress),
}

fn for_each_requirement<'a>(
    rule: &'a ScryptoAccessRule,
    f: &mut impl FnMut(RuleLeaf<'a>),
) {
    fn composite<'a>(
        requirement: &'a ScryptoCompositeRequirement,
        f: &mut impl FnMut(RuleLeaf<'a>),
    ) {
        match requirement {
            ScryptoCompositeRequirement::BasicRequirement(basic) => match basic
            {
                ScryptoBasicRequirement::Require(badge) => {
                    f(RuleLeaf::Require(badge))
                }
                ScryptoBasicRequirement::AmountOf(amount, badge) => {
                    f(RuleLeaf::AmountOf(*amount, badge))
                }
                ScryptoBasicRequirement::CountOf(_, list)
                | ScryptoBasicRequirement::AllOf(list)
                | ScryptoBasicRequirement::AnyOf(list) => {
                    list.iter().for_each(|badge| f(RuleLeaf::Require(badge)))
                }
            },
            ScryptoCompositeRequirement::AnyOf(list)
            | ScryptoCompositeRequirement::AllOf(list) => {
                list.iter().for_each(|r| composite(r, f))
            }
        }
    }
    if let ScryptoAccessRule::Protected(requirement) = rule {
        composite(requirement, f)
    }
}
