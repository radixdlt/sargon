use crate::prelude::*;

/// The proofs present when an access rule is checked, as in the engine's auth
/// zone.
///
/// It is flat: the engine also checks the auth zones of the caller, which a
/// caller adds here. It doesn't model preview's "assume all signature proofs",
/// which proves every non-fungible of a resource.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuthZone {
    /// Non-fungibles proven without a proof, e.g. the signature badges of a
    /// transaction's signers.
    pub implicit_non_fungibles: HashSet<NonFungibleGlobalId>,

    /// Proofs created from vaults or buckets, e.g. by `create_proof_of_amount`.
    pub proofs: Vec<AuthZoneProof>,
}

/// A proof of an amount of a resource, or of some of its non-fungibles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthZoneProof {
    pub resource: ResourceAddress,
    pub amount: Decimal192,
    pub non_fungible_local_ids: HashSet<NonFungibleLocalId>,
}

impl AccessRequirement {
    /// Whether the proofs in `auth_zone` pass this requirement, checked the
    /// way the engine checks it (`Authorization::verify_proof_rule`).
    pub fn is_satisfied_by(&self, auth_zone: &AuthZone) -> bool {
        match self {
            Self::Anyone => true,
            Self::NoOne => false,
            Self::Badge { badge } => auth_zone.has_badge(badge),
            Self::AmountOf { amount, resource } => {
                auth_zone.has_amount(*amount, resource)
            }
            Self::AtLeast {
                count,
                requirements,
            } => {
                let satisfied = requirements
                    .iter()
                    .filter(|requirement| {
                        requirement.is_satisfied_by(auth_zone)
                    })
                    .count();
                satisfied >= *count as usize
            }
        }
    }
}

impl AuthZone {
    /// A non-fungible is proven implicitly or by a proof that contains it. A
    /// resource is proven by any proof of it, but not by implicit proofs.
    fn has_badge(&self, badge: &ResourceOrNonFungible) -> bool {
        match badge {
            ResourceOrNonFungible::NonFungible { value } => {
                self.implicit_non_fungibles.contains(value)
                    || self.proofs.iter().any(|proof| {
                        proof.resource == value.resource_address
                            && proof
                                .non_fungible_local_ids
                                .contains(&value.non_fungible_local_id)
                    })
            }
            ResourceOrNonFungible::Resource { value } => {
                self.proofs.iter().any(|proof| proof.resource == *value)
            }
        }
    }

    /// The engine checks each proof on its own, so smaller proofs of the same
    /// resource don't add up.
    fn has_amount(
        &self,
        amount: Decimal192,
        resource: &ResourceAddress,
    ) -> bool {
        self.proofs
            .iter()
            .any(|proof| proof.resource == *resource && proof.amount >= amount)
    }
}
