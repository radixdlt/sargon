use crate::prelude::*;

/// What an account holds of the resources a withdraw rule may ask a proof of: the amount of
/// each fungible resource, and some of the ids of each non-fungible resource (enough to prove
/// them, not necessarily all of them).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProofHoldings {
    pub fungibles: HashMap<ResourceAddress, Decimal192>,
    pub non_fungibles: HashMap<ResourceAddress, Vec<NonFungibleLocalId>>,
}

impl ProofHoldings {
    pub fn new(
        fungibles: impl IntoIterator<Item = (ResourceAddress, Decimal192)>,
        non_fungibles: impl IntoIterator<
            Item = (ResourceAddress, Vec<NonFungibleLocalId>),
        >,
    ) -> Self {
        Self {
            fungibles: fungibles.into_iter().collect(),
            non_fungibles: non_fungibles.into_iter().collect(),
        }
    }

    fn fungible_amount(&self, resource: &ResourceAddress) -> Decimal192 {
        self.fungibles
            .get(resource)
            .cloned()
            .unwrap_or_else(Decimal192::zero)
    }

    fn ids(&self, resource: &ResourceAddress) -> &[NonFungibleLocalId] {
        self.non_fungibles
            .get(resource)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    /// A proof that the account holds some of `resource`, if it does.
    fn proof_of_resource(
        &self,
        resource: &ResourceAddress,
    ) -> Option<ResourceSpecifier> {
        if let Some(id) = self.ids(resource).first() {
            return Some(ResourceSpecifier::non_fungible(
                *resource,
                vec![id.clone()],
            ));
        }
        let owned = self.fungible_amount(resource);
        if !owned.is_positive() {
            return None;
        }
        // One is the usual badge amount; a smaller balance proves the resource just as well.
        let amount = if owned >= Decimal192::one() {
            Decimal192::one()
        } else {
            owned
        };
        Some(ResourceSpecifier::fungible(*resource, amount))
    }

    /// A proof of at least `amount` of `resource`, if the account holds that much.
    fn proof_of_amount(
        &self,
        resource: &ResourceAddress,
        amount: Decimal192,
    ) -> Option<ResourceSpecifier> {
        let ids = self.ids(resource);
        if !ids.is_empty() {
            let proven = |taken: &Vec<NonFungibleLocalId>| {
                Decimal192::from(taken.len() as u64)
            };
            let mut taken = Vec::new();
            for id in ids {
                if proven(&taken) >= amount {
                    break;
                }
                taken.push(id.clone());
            }
            return (proven(&taken) >= amount && !taken.is_empty())
                .then(|| ResourceSpecifier::non_fungible(*resource, taken));
        }
        (self.fungible_amount(resource) >= amount && amount.is_positive())
            .then(|| ResourceSpecifier::fungible(*resource, amount))
    }

    /// A proof of the non-fungible `id` of `resource`, if the account holds it.
    fn proof_of_non_fungible(
        &self,
        resource: &ResourceAddress,
        id: &NonFungibleLocalId,
    ) -> Option<ResourceSpecifier> {
        self.ids(resource).contains(id).then(|| {
            ResourceSpecifier::non_fungible(*resource, vec![id.clone()])
        })
    }
}

impl Requirement {
    fn resource(&self) -> ResourceAddress {
        match self {
            Requirement::Resource { resource } => *resource,
            Requirement::NonFungible { non_fungible } => {
                non_fungible.resource_address
            }
        }
    }

    fn proof(&self, holdings: &ProofHoldings) -> Option<ResourceSpecifier> {
        match self {
            Requirement::Resource { resource } => {
                holdings.proof_of_resource(resource)
            }
            Requirement::NonFungible { non_fungible } => {
                let id = non_fungible.local_id.simple_rep.parse().ok()?;
                holdings
                    .proof_of_non_fungible(&non_fungible.resource_address, &id)
            }
        }
    }
}

/// The proofs of `count` of the `requirements` that the account can satisfy, if it can
/// satisfy that many.
fn proofs_of_count(
    count: usize,
    proofs: impl IntoIterator<Item = Option<Vec<ResourceSpecifier>>>,
) -> Option<Vec<ResourceSpecifier>> {
    let satisfied = proofs.into_iter().flatten().take(count).collect_vec();
    (satisfied.len() >= count)
        .then(|| satisfied.into_iter().flatten().collect())
}

impl BasicRequirement {
    fn resources(&self) -> Vec<ResourceAddress> {
        match self {
            BasicRequirement::Require { requirement } => {
                vec![requirement.resource()]
            }
            BasicRequirement::AmountOf { resource, .. } => vec![*resource],
            BasicRequirement::AllOf { list }
            | BasicRequirement::AnyOf { list }
            | BasicRequirement::CountOf { list, .. } => {
                list.iter().map(Requirement::resource).collect()
            }
        }
    }

    fn proofs(
        &self,
        holdings: &ProofHoldings,
    ) -> Option<Vec<ResourceSpecifier>> {
        let each = |list: &[Requirement]| {
            list.iter()
                .map(|r| r.proof(holdings).map(|proof| vec![proof]))
                .collect_vec()
        };
        match self {
            BasicRequirement::Require { requirement } => {
                requirement.proof(holdings).map(|proof| vec![proof])
            }
            BasicRequirement::AmountOf { amount, resource } => holdings
                .proof_of_amount(resource, *amount)
                .map(|proof| vec![proof]),
            BasicRequirement::AllOf { list } => {
                proofs_of_count(list.len(), each(list))
            }
            BasicRequirement::AnyOf { list } => {
                each(list).into_iter().flatten().next()
            }
            BasicRequirement::CountOf { count, list } => {
                proofs_of_count(*count as usize, each(list))
            }
        }
    }
}

impl CompositeRequirement {
    fn resources(&self) -> Vec<ResourceAddress> {
        match self {
            CompositeRequirement::ProofRule { proof_rule } => {
                proof_rule.resources()
            }
            CompositeRequirement::AnyOf { access_rules }
            | CompositeRequirement::AllOf { access_rules } => {
                access_rules.iter().flat_map(Self::resources).collect()
            }
        }
    }

    fn proofs(
        &self,
        holdings: &ProofHoldings,
    ) -> Option<Vec<ResourceSpecifier>> {
        match self {
            CompositeRequirement::ProofRule { proof_rule } => {
                proof_rule.proofs(holdings)
            }
            CompositeRequirement::AnyOf { access_rules } => {
                access_rules.iter().find_map(|rule| rule.proofs(holdings))
            }
            CompositeRequirement::AllOf { access_rules } => proofs_of_count(
                access_rules.len(),
                access_rules.iter().map(|rule| rule.proofs(holdings)),
            ),
        }
    }
}

impl AccessRule {
    /// The resources this rule may ask proofs of: the badges to look for in the account.
    pub fn proof_resources(&self) -> Vec<ResourceAddress> {
        match self {
            AccessRule::Protected { access_rule } => {
                access_rule.resources().into_iter().unique().collect()
            }
            AccessRule::AllowAll | AccessRule::DenyAll => Vec::new(),
        }
    }

    /// The proofs to create from an account holding `holdings` so that this rule is satisfied:
    /// none for `AllowAll`, `None` when it cannot be (`DenyAll`, or missing badges).
    pub fn proofs_satisfying(
        &self,
        holdings: &ProofHoldings,
    ) -> Option<Vec<ResourceSpecifier>> {
        match self {
            AccessRule::AllowAll => Some(Vec::new()),
            AccessRule::DenyAll => None,
            AccessRule::Protected { access_rule } => {
                access_rule.proofs(holdings).map(merge_proofs)
            }
        }
    }
}

/// One proof per resource: the largest amount, or all the ids, asked of it.
pub fn merge_proofs(
    proofs: impl IntoIterator<Item = ResourceSpecifier>,
) -> Vec<ResourceSpecifier> {
    let mut merged: IndexMap<ResourceAddress, ResourceSpecifier> =
        IndexMap::new();
    for proof in proofs {
        let resource = proof.get_address();
        let combined = match (merged.swap_remove(&resource), proof) {
            (
                Some(ResourceSpecifier::Fungible { amount: a, .. }),
                ResourceSpecifier::Fungible { amount: b, .. },
            ) => ResourceSpecifier::fungible(
                resource,
                if a >= b { a } else { b },
            ),
            (
                Some(ResourceSpecifier::NonFungible { ids: a, .. }),
                ResourceSpecifier::NonFungible { ids: b, .. },
            ) => ResourceSpecifier::non_fungible(
                resource,
                a.into_iter().chain(b).unique().collect(),
            ),
            (_, proof) => proof,
        };
        merged.insert(resource, combined);
    }
    merged.into_values().collect()
}

impl StateEntityDetailsResponseItemDetails {
    /// The rule who may withdraw this resource from a vault: the `withdrawer` role of its main
    /// module, or its owner's rule when the role is assigned to the owner. `None` when it is not
    /// a resource.
    pub fn withdrawer_rule(&self) -> Option<AccessRule> {
        let role_assignments = match self {
            Self::FungibleResource(details) => &details.role_assignments,
            Self::NonFungibleResource(details) => &details.role_assignments,
            _ => return None,
        };
        let withdrawer = role_assignments
            .entries
            .iter()
            .find(|entry| entry.role_key == RoleKey::main_withdrawer())?;
        match withdrawer.assignment.resolution {
            RoleAssignmentResolution::Owner => {
                Some(role_assignments.owner.rule.clone())
            }
            RoleAssignmentResolution::Explicit => {
                withdrawer.assignment.explicit_rule.clone()
            }
        }
    }
}

/// A transferred resource whose withdrawal is restricted: it needs proofs of some badges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RestrictedWithdrawal {
    /// The transferred resource.
    pub resource: ResourceAddress,
    /// The badges its withdraw rule asks proofs of. Empty when nobody may withdraw it.
    pub badges: Vec<ResourceAddress>,
    /// Whether the account holds what the rule asks for.
    pub is_satisfied: bool,
}

/// What the withdrawals of a transfer need: the proofs to create before withdrawing (for the
/// manifest), and the restricted resources with their badges (for the person).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TransferWithdrawProofs {
    pub proofs: Vec<ResourceSpecifier>,
    pub restricted_withdrawals: Vec<RestrictedWithdrawal>,
}

impl TransferWithdrawProofs {
    /// From the withdraw rule of each transferred resource (those that anybody may withdraw can
    /// be left out) and what the account holds.
    pub fn new(
        withdraw_rules: impl IntoIterator<Item = (ResourceAddress, AccessRule)>,
        holdings: &ProofHoldings,
    ) -> Self {
        let mut proofs = Vec::new();
        let mut restricted_withdrawals = Vec::new();
        for (resource, rule) in withdraw_rules {
            if rule == AccessRule::AllowAll {
                continue;
            }
            let satisfying = rule.proofs_satisfying(holdings);
            restricted_withdrawals.push(RestrictedWithdrawal {
                resource,
                badges: rule.proof_resources(),
                is_satisfied: satisfying.is_some(),
            });
            proofs.extend(satisfying.unwrap_or_default());
        }
        Self {
            proofs: merge_proofs(proofs),
            restricted_withdrawals,
        }
    }

    /// Whether every restricted withdrawal can be done.
    pub fn is_satisfied(&self) -> bool {
        self.restricted_withdrawals.iter().all(|w| w.is_satisfied)
    }
}

impl HasSampleValues for RestrictedWithdrawal {
    fn sample() -> Self {
        Self {
            resource: ResourceAddress::sample_mainnet_xrd(),
            badges: vec![ResourceAddress::sample_mainnet_candy()],
            is_satisfied: true,
        }
    }

    fn sample_other() -> Self {
        Self {
            resource: ResourceAddress::sample_mainnet_nft_gc_membership(),
            badges: Vec::new(),
            is_satisfied: false,
        }
    }
}

impl HasSampleValues for TransferWithdrawProofs {
    fn sample() -> Self {
        Self {
            proofs: vec![ResourceSpecifier::fungible(
                ResourceAddress::sample_mainnet_candy(),
                Decimal192::one(),
            )],
            restricted_withdrawals: vec![RestrictedWithdrawal::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            proofs: Vec::new(),
            restricted_withdrawals: vec![RestrictedWithdrawal::sample_other()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_candy()
    }

    fn nft_badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_nft_other()
    }

    fn protected(rule: CompositeRequirement) -> AccessRule {
        AccessRule::Protected { access_rule: rule }
    }

    fn require(requirement: Requirement) -> CompositeRequirement {
        CompositeRequirement::ProofRule {
            proof_rule: BasicRequirement::Require { requirement },
        }
    }

    fn require_resource(resource: ResourceAddress) -> CompositeRequirement {
        require(Requirement::Resource { resource })
    }

    fn require_nft(
        resource: ResourceAddress,
        id: &str,
    ) -> CompositeRequirement {
        require(Requirement::NonFungible {
            non_fungible: NonFungible {
                resource_address: resource,
                local_id: NonFungibleLocalIdId {
                    id_type: "Integer".to_owned(),
                    sbor_hex: String::new(),
                    simple_rep: id.to_owned(),
                },
            },
        })
    }

    fn holding_badge(amount: u64) -> ProofHoldings {
        ProofHoldings::new([(badge(), Decimal192::from(amount))], [])
    }

    fn holding_nfts(ids: &[u64]) -> ProofHoldings {
        ProofHoldings::new(
            [],
            [(
                nft_badge(),
                ids.iter()
                    .map(|i| NonFungibleLocalId::integer(*i))
                    .collect(),
            )],
        )
    }

    #[test]
    fn allow_all_needs_no_proof_and_deny_all_cannot_be_satisfied() {
        let holdings = holding_badge(1);
        assert_eq!(
            AccessRule::AllowAll.proofs_satisfying(&holdings),
            Some(vec![])
        );
        assert_eq!(AccessRule::DenyAll.proofs_satisfying(&holdings), None);
        assert!(AccessRule::AllowAll.proof_resources().is_empty());
    }

    #[test]
    fn a_fungible_badge_is_proven_with_one() {
        let rule = protected(require_resource(badge()));
        assert_eq!(rule.proof_resources(), vec![badge()]);
        assert_eq!(
            rule.proofs_satisfying(&holding_badge(5)),
            Some(vec![ResourceSpecifier::fungible(
                badge(),
                Decimal192::one()
            )])
        );
        assert_eq!(rule.proofs_satisfying(&holding_badge(0)), None);
        assert_eq!(rule.proofs_satisfying(&ProofHoldings::default()), None);
    }

    #[test]
    fn a_non_fungible_resource_badge_is_proven_with_one_of_its_ids() {
        let rule = protected(require_resource(nft_badge()));
        assert_eq!(
            rule.proofs_satisfying(&holding_nfts(&[7, 8])),
            Some(vec![ResourceSpecifier::non_fungible(
                nft_badge(),
                vec![NonFungibleLocalId::integer(7)]
            )])
        );
    }

    #[test]
    fn a_specific_non_fungible_must_be_held() {
        let rule = protected(require_nft(nft_badge(), "#8#"));
        assert_eq!(
            rule.proofs_satisfying(&holding_nfts(&[7, 8])),
            Some(vec![ResourceSpecifier::non_fungible(
                nft_badge(),
                vec![NonFungibleLocalId::integer(8)]
            )])
        );
        assert_eq!(rule.proofs_satisfying(&holding_nfts(&[7])), None);
    }

    #[test]
    fn amount_of() {
        let rule = protected(CompositeRequirement::ProofRule {
            proof_rule: BasicRequirement::AmountOf {
                amount: Decimal192::from(3u64),
                resource: badge(),
            },
        });
        assert_eq!(
            rule.proofs_satisfying(&holding_badge(4)),
            Some(vec![ResourceSpecifier::fungible(
                badge(),
                Decimal192::from(3u64)
            )])
        );
        assert_eq!(rule.proofs_satisfying(&holding_badge(2)), None);

        let nft_rule = protected(CompositeRequirement::ProofRule {
            proof_rule: BasicRequirement::AmountOf {
                amount: Decimal192::from(2u64),
                resource: nft_badge(),
            },
        });
        assert_eq!(
            nft_rule.proofs_satisfying(&holding_nfts(&[1, 2, 3])),
            Some(vec![ResourceSpecifier::non_fungible(
                nft_badge(),
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2)
                ]
            )])
        );
        assert_eq!(nft_rule.proofs_satisfying(&holding_nfts(&[1])), None);
    }

    #[test]
    fn any_of_takes_the_first_badge_held_and_all_of_needs_every_one() {
        let any = protected(CompositeRequirement::AnyOf {
            access_rules: vec![
                require_resource(nft_badge()),
                require_resource(badge()),
            ],
        });
        assert_eq!(any.proof_resources(), vec![nft_badge(), badge()]);
        assert_eq!(
            any.proofs_satisfying(&holding_badge(1)),
            Some(vec![ResourceSpecifier::fungible(
                badge(),
                Decimal192::one()
            )])
        );

        let all = protected(CompositeRequirement::AllOf {
            access_rules: vec![
                require_resource(nft_badge()),
                require_resource(badge()),
            ],
        });
        assert_eq!(all.proofs_satisfying(&holding_badge(1)), None);
        let both = ProofHoldings::new(
            [(badge(), Decimal192::one())],
            [(nft_badge(), vec![NonFungibleLocalId::integer(1)])],
        );
        assert_eq!(all.proofs_satisfying(&both).map(|p| p.len()), Some(2));
    }

    #[test]
    fn count_of() {
        let rule = protected(CompositeRequirement::ProofRule {
            proof_rule: BasicRequirement::CountOf {
                count: 1,
                list: vec![
                    Requirement::Resource {
                        resource: nft_badge(),
                    },
                    Requirement::Resource { resource: badge() },
                ],
            },
        });
        assert_eq!(
            rule.proofs_satisfying(&holding_badge(1)),
            Some(vec![ResourceSpecifier::fungible(
                badge(),
                Decimal192::one()
            )])
        );
        assert_eq!(rule.proofs_satisfying(&ProofHoldings::default()), None);
    }

    #[test]
    fn proofs_of_the_same_resource_are_merged() {
        let merged = merge_proofs([
            ResourceSpecifier::fungible(badge(), Decimal192::one()),
            ResourceSpecifier::fungible(badge(), Decimal192::from(2u64)),
            ResourceSpecifier::non_fungible(
                nft_badge(),
                vec![NonFungibleLocalId::integer(1)],
            ),
            ResourceSpecifier::non_fungible(
                nft_badge(),
                vec![
                    NonFungibleLocalId::integer(1),
                    NonFungibleLocalId::integer(2),
                ],
            ),
        ]);
        assert_eq!(
            merged,
            vec![
                ResourceSpecifier::fungible(badge(), Decimal192::from(2u64)),
                ResourceSpecifier::non_fungible(
                    nft_badge(),
                    vec![
                        NonFungibleLocalId::integer(1),
                        NonFungibleLocalId::integer(2)
                    ]
                ),
            ]
        );
    }

    #[test]
    fn withdrawer_rule_follows_the_owner_when_assigned_to_it() {
        let details = StateEntityDetailsResponseItemDetails::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                ComponentEntityRoleAssignments::sample_deny_all(),
            ),
        );
        assert_eq!(details.withdrawer_rule(), Some(AccessRule::DenyAll));

        let owner_rule = protected(require_resource(badge()));
        let details = StateEntityDetailsResponseItemDetails::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                ComponentEntityRoleAssignments::new(
                    ComponentEntityRoleAssignmentOwner::new(owner_rule.clone()),
                    [ComponentEntityRoleAssignmentEntry::sample_withdrawer_owner_allow_all()],
                ),
            ),
        );
        assert_eq!(details.withdrawer_rule(), Some(owner_rule));
    }

    #[test]
    fn transfer_proofs_cover_every_restricted_resource() {
        let restricted = ResourceAddress::sample_mainnet_xrd();
        let forbidden = ResourceAddress::sample_mainnet_nft_gc_membership();
        let sut = TransferWithdrawProofs::new(
            [
                (badge(), AccessRule::AllowAll),
                (restricted, protected(require_resource(badge()))),
            ],
            &holding_badge(1),
        );
        assert!(sut.is_satisfied());
        assert_eq!(
            sut.proofs,
            vec![ResourceSpecifier::fungible(badge(), Decimal192::one())]
        );
        assert_eq!(
            sut.restricted_withdrawals,
            vec![RestrictedWithdrawal {
                resource: restricted,
                badges: vec![badge()],
                is_satisfied: true
            }]
        );

        let sut = TransferWithdrawProofs::new(
            [(forbidden, AccessRule::DenyAll)],
            &holding_badge(1),
        );
        assert!(!sut.is_satisfied());
        assert!(sut.proofs.is_empty());
        assert!(sut.restricted_withdrawals[0].badges.is_empty());
    }
}
