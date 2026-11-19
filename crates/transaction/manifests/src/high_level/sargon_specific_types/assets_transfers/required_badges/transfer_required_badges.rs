use crate::prelude::*;
use radix_engine_interface::blueprints::resource::{
    BasicRequirement as ScryptoBasicRequirement,
    CompositeRequirement as ScryptoCompositeRequirement,
    ResourceOrNonFungible as ScryptoResourceOrNonFungible,
};

/// What the withdrawals of a transfer need: the badges to prove before them (for the manifest),
/// and the restricted resources with what their rules ask for (for the person).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TransferRequiredBadges {
    /// The badges to prove before withdrawing the restricted resources: give them to
    /// `per_asset_transfers` / `per_recipient_transfers`.
    pub required_badges: Vec<ResourceSpecifier>,

    /// The transferred resources that not everybody may withdraw.
    pub restricted_withdrawals: Vec<RestrictedWithdrawal>,
}

impl TransferRequiredBadges {
    /// Matches the withdraw rules against what the account can show: the badges it holds, and
    /// the signatures of its keys (`signer_public_keys`, which surely sign the transfer, so their
    /// signatures are in the auth zone without any proof; as in the engine, they only satisfy a
    /// `Require` of that very signature).
    ///
    /// When a rule can be satisfied in several ways, the one needing the fewest proofs is chosen,
    /// preferring the badges already proven for the other resources.
    ///
    /// A restricted resource that is also one of the required badges is `NotWithdrawable` in
    /// this transfer: the manifest withdraws the badges after dropping the proofs (a proof locks
    /// the badge), so the badge's own rule could not be satisfied then.
    pub fn new(
        withdraw_rules: &WithdrawRules,
        signer_public_keys: impl IntoIterator<Item = PublicKey>,
        holdings: &BadgeHoldings,
    ) -> Self {
        let signatures = signer_public_keys
            .into_iter()
            .map(|key| {
                ScryptoNonFungibleGlobalId::from_public_key(
                    ScryptoPublicKey::from(key),
                )
            })
            .collect::<IndexSet<_>>();

        let restricted = withdraw_rules
            .iter()
            .map(|(resource, _)| *resource)
            .collect::<IndexSet<_>>();
        let mut proven = BadgeProofs::default();
        let mut restricted_withdrawals = Vec::new();
        for (resource, rule) in withdraw_rules.iter() {
            let network_id = resource.network_id();
            let outcome = Matcher {
                network_id,
                signatures: &signatures,
                holdings,
                proven: &proven,
                restricted: &restricted,
            }
            .access_rule(rule);
            let status = match outcome {
                Outcome::Satisfied(proofs) => {
                    proven.merge(proofs);
                    RestrictedWithdrawalStatus::Satisfied
                }
                Outcome::MissingBadges => {
                    RestrictedWithdrawalStatus::MissingBadges
                }
                Outcome::Unsatisfiable => {
                    RestrictedWithdrawalStatus::NotWithdrawable
                }
            };
            restricted_withdrawals.push(RestrictedWithdrawal::new(
                *resource,
                badges_of(rule, network_id),
                status,
            ));
        }

        for withdrawal in restricted_withdrawals.iter_mut() {
            if proven.0.contains_key(&withdrawal.resource) {
                withdrawal.status = RestrictedWithdrawalStatus::NotWithdrawable;
            }
        }

        Self {
            required_badges: proven.into_resource_specifiers(),
            restricted_withdrawals,
        }
    }

    /// Whether every restricted withdrawal can be done.
    pub fn is_satisfied(&self) -> bool {
        self.restricted_withdrawals
            .iter()
            .all(|w| w.status == RestrictedWithdrawalStatus::Satisfied)
    }
}

impl HasSampleValues for TransferRequiredBadges {
    fn sample() -> Self {
        Self {
            required_badges: vec![ResourceSpecifier::fungible(
                ResourceAddress::sample_mainnet_candy(),
                Decimal192::one(),
            )],
            restricted_withdrawals: vec![RestrictedWithdrawal::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            required_badges: Vec::new(),
            restricted_withdrawals: vec![RestrictedWithdrawal::sample_other()],
        }
    }
}

/// The proofs to create: of an amount of each fungible badge, of some ids of each non-fungible
/// one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct BadgeProofs(IndexMap<ResourceAddress, BadgeProof>);

#[derive(Clone, Debug, PartialEq, Eq)]
enum BadgeProof {
    Fungible(Decimal192),
    NonFungible(IndexSet<NonFungibleLocalId>),
}

impl BadgeProofs {
    fn just(resource: ResourceAddress, proof: BadgeProof) -> Self {
        Self(IndexMap::from_iter([(resource, proof)]))
    }

    /// One proof per badge, covering both: the largest amount, all the ids.
    fn merge(&mut self, other: Self) {
        for (resource, proof) in other.0 {
            let merged = match (self.0.swap_remove(&resource), proof) {
                (Some(BadgeProof::Fungible(a)), BadgeProof::Fungible(b)) => {
                    BadgeProof::Fungible(if a >= b { a } else { b })
                }
                (
                    Some(BadgeProof::NonFungible(mut a)),
                    BadgeProof::NonFungible(b),
                ) => {
                    a.extend(b);
                    BadgeProof::NonFungible(a)
                }
                (_, proof) => proof,
            };
            self.0.insert(resource, merged);
        }
    }

    /// What these proofs add to `proven`: the badges not proven yet, then the ids not proven
    /// yet. Used to choose the cheapest way to satisfy a rule.
    fn cost(&self, proven: &Self) -> (usize, usize) {
        let new_badges = self
            .0
            .keys()
            .filter(|resource| !proven.0.contains_key(*resource))
            .count();
        let new_ids = self
            .0
            .iter()
            .map(|(resource, proof)| match (proof, proven.0.get(resource)) {
                (
                    BadgeProof::NonFungible(ids),
                    Some(BadgeProof::NonFungible(proven_ids)),
                ) => ids.difference(proven_ids).count(),
                (BadgeProof::NonFungible(ids), _) => ids.len(),
                (BadgeProof::Fungible(_), _) => 0,
            })
            .sum();
        (new_badges, new_ids)
    }

    fn into_resource_specifiers(self) -> Vec<ResourceSpecifier> {
        self.0
            .into_iter()
            .map(|(resource, proof)| match proof {
                BadgeProof::Fungible(amount) => {
                    ResourceSpecifier::fungible(resource, amount)
                }
                BadgeProof::NonFungible(ids) => {
                    ResourceSpecifier::non_fungible(
                        resource,
                        ids.into_iter().collect(),
                    )
                }
            })
            .collect()
    }
}

/// What the account can do about (a part of) a rule.
enum Outcome {
    /// It satisfies it with these proofs (none when signing is enough).
    Satisfied(BadgeProofs),
    /// It would satisfy it if it held (more of) the badges asked for.
    MissingBadges,
    /// No badge it could hold satisfies it.
    Unsatisfiable,
}

impl Outcome {
    fn nothing_needed() -> Self {
        Self::Satisfied(BadgeProofs::default())
    }
}

struct Matcher<'a> {
    network_id: NetworkID,
    signatures: &'a IndexSet<ScryptoNonFungibleGlobalId>,
    holdings: &'a BadgeHoldings,
    /// The proofs already chosen for the other resources of the transfer.
    proven: &'a BadgeProofs,
    /// The restricted resources being transferred: a poor choice of badge, since they become
    /// `NotWithdrawable` when they are one.
    restricted: &'a IndexSet<ResourceAddress>,
}

impl Matcher<'_> {
    fn access_rule(&self, rule: &ScryptoAccessRule) -> Outcome {
        match rule {
            ScryptoAccessRule::AllowAll => Outcome::nothing_needed(),
            ScryptoAccessRule::DenyAll => Outcome::Unsatisfiable,
            ScryptoAccessRule::Protected(requirement) => {
                self.composite(requirement)
            }
        }
    }

    fn composite(&self, requirement: &ScryptoCompositeRequirement) -> Outcome {
        match requirement {
            ScryptoCompositeRequirement::BasicRequirement(basic) => {
                self.basic(basic)
            }
            ScryptoCompositeRequirement::AllOf(list) => self
                .count_of(list.len(), list.iter().map(|r| self.composite(r))),
            ScryptoCompositeRequirement::AnyOf(list) => {
                self.count_of(1, list.iter().map(|r| self.composite(r)))
            }
        }
    }

    fn basic(&self, requirement: &ScryptoBasicRequirement) -> Outcome {
        match requirement {
            ScryptoBasicRequirement::Require(badge) => self.require(badge),
            ScryptoBasicRequirement::AmountOf(amount, badge) => {
                self.amount_of(*amount, badge)
            }
            ScryptoBasicRequirement::CountOf(count, list) => self.count_of(
                *count as usize,
                list.iter().map(|badge| self.require(badge)),
            ),
            ScryptoBasicRequirement::AllOf(list) => self.count_of(
                list.len(),
                list.iter().map(|badge| self.require(badge)),
            ),
            ScryptoBasicRequirement::AnyOf(list) => {
                self.count_of(1, list.iter().map(|badge| self.require(badge)))
            }
        }
    }

    /// `count` of the `outcomes` must be satisfied: the cheapest ones are chosen.
    fn count_of(
        &self,
        count: usize,
        outcomes: impl IntoIterator<Item = Outcome>,
    ) -> Outcome {
        let mut satisfied = Vec::new();
        let mut missing = 0;
        for outcome in outcomes {
            match outcome {
                Outcome::Satisfied(proofs) => satisfied.push(proofs),
                Outcome::MissingBadges => missing += 1,
                Outcome::Unsatisfiable => {}
            }
        }
        if satisfied.len() >= count {
            satisfied.sort_by_key(|proofs| {
                let restricted_badges = proofs
                    .0
                    .keys()
                    .filter(|badge| self.restricted.contains(*badge))
                    .count();
                (restricted_badges, proofs.cost(self.proven))
            });
            let mut proofs = BadgeProofs::default();
            satisfied
                .into_iter()
                .take(count)
                .for_each(|p| proofs.merge(p));
            Outcome::Satisfied(proofs)
        } else if satisfied.len() + missing >= count {
            Outcome::MissingBadges
        } else {
            Outcome::Unsatisfiable
        }
    }

    fn require(&self, badge: &ScryptoResourceOrNonFungible) -> Outcome {
        match badge {
            ScryptoResourceOrNonFungible::NonFungible(id) => {
                self.require_non_fungible(id)
            }
            ScryptoResourceOrNonFungible::Resource(resource) => {
                if is_implicit(resource) {
                    // The engine only accepts the implicit proofs (signatures, callers) for a
                    // `Require` of a specific non-fungible.
                    return Outcome::Unsatisfiable;
                }
                let resource = self.resource(resource);
                if resource.is_fungible() {
                    // Any amount proves the resource; one is the usual badge amount.
                    let held = self.holdings.amount_of(&resource);
                    if !held.is_positive() {
                        return Outcome::MissingBadges;
                    }
                    let amount = if held >= Decimal192::one() {
                        Decimal192::one()
                    } else {
                        held
                    };
                    Outcome::Satisfied(BadgeProofs::just(
                        resource,
                        BadgeProof::Fungible(amount),
                    ))
                } else {
                    self.non_fungibles_of(resource, 1)
                }
            }
        }
    }

    fn require_non_fungible(&self, id: &ScryptoNonFungibleGlobalId) -> Outcome {
        if is_implicit(&id.resource_address()) {
            return if self.signatures.contains(id) {
                Outcome::nothing_needed()
            } else {
                // Another key's signature, or a component or package call: not a transfer's.
                Outcome::Unsatisfiable
            };
        }
        let resource = self.resource(&id.resource_address());
        let local_id = NonFungibleLocalId::from(id.local_id().clone());
        if self.holdings.ids_of(&resource).contains(&local_id) {
            Outcome::Satisfied(BadgeProofs::just(
                resource,
                BadgeProof::NonFungible(IndexSet::just(local_id)),
            ))
        } else {
            Outcome::MissingBadges
        }
    }

    fn amount_of(
        &self,
        amount: ScryptoDecimal192,
        resource: &ScryptoResourceAddress,
    ) -> Outcome {
        if is_implicit(resource) {
            // The engine checks the amount against real proofs only.
            return Outcome::Unsatisfiable;
        }
        if !amount.is_positive() {
            // The engine still wants a proof of the resource.
            return self
                .require(&ScryptoResourceOrNonFungible::Resource(*resource));
        }
        let resource = self.resource(resource);
        if resource.is_fungible() {
            let held = self.holdings.amount_of(&resource);
            if held < Decimal192::from(amount) {
                return Outcome::MissingBadges;
            }
            // A fractional amount may not fit the badge's divisibility (which is not known
            // here): prove the whole balance instead, which always does, and is enough.
            let is_whole = amount.checked_floor() == Some(amount);
            let proven = if is_whole {
                Decimal192::from(amount)
            } else {
                held
            };
            Outcome::Satisfied(BadgeProofs::just(
                resource,
                BadgeProof::Fungible(proven),
            ))
        } else {
            self.non_fungibles_of(resource, ids_needed_for(amount))
        }
    }

    /// A proof of `count` ids of `resource`, preferring those already proven.
    fn non_fungibles_of(
        &self,
        resource: ResourceAddress,
        count: u64,
    ) -> Outcome {
        let held = self.holdings.ids_of(&resource);
        if (held.len() as u64) < count {
            return Outcome::MissingBadges;
        }
        let already_proven = match self.proven.0.get(&resource) {
            Some(BadgeProof::NonFungible(ids)) => ids.clone(),
            _ => IndexSet::new(),
        };
        let (proven_first, others): (Vec<_>, Vec<_>) =
            held.into_iter().partition(|id| already_proven.contains(id));
        let ids = proven_first
            .into_iter()
            .chain(others)
            .take(count as usize)
            .collect();
        Outcome::Satisfied(BadgeProofs::just(
            resource,
            BadgeProof::NonFungible(ids),
        ))
    }

    fn resource(&self, resource: &ScryptoResourceAddress) -> ResourceAddress {
        (*resource, self.network_id).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_common::constants::{
        GLOBAL_CALLER_RESOURCE, PACKAGE_OF_DIRECT_CALLER_RESOURCE,
        SECP256K1_SIGNATURE_RESOURCE,
    };

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransferRequiredBadges;

    // === Resources ===

    /// The restricted token being transferred.
    fn token() -> ResourceAddress {
        ResourceAddress::new_from_node_id(
            ScryptoNodeId::new(
                ScryptoEntityType::GlobalFungibleResourceManager as u8,
                &[7u8; 29],
            ),
            NetworkID::Mainnet,
        )
        .unwrap()
    }

    /// Another restricted token being transferred.
    fn other_token() -> ResourceAddress {
        ResourceAddress::sample_mainnet_nft_gc_membership()
    }

    fn badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_candy()
    }

    fn other_badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet()
    }

    fn nft_badge() -> ResourceAddress {
        ResourceAddress::sample_mainnet_nft_other()
    }

    fn implicit(resource: ScryptoResourceAddress) -> ResourceAddress {
        (resource, NetworkID::Mainnet).into()
    }

    fn id(i: u64) -> NonFungibleLocalId {
        NonFungibleLocalId::integer(i)
    }

    // === Rules, as read from the Gateway ===

    fn resource_requirement(resource: ResourceAddress) -> Requirement {
        Requirement::Resource { resource }
    }

    fn non_fungible_requirement(id: NonFungibleGlobalId) -> Requirement {
        Requirement::NonFungible {
            non_fungible: NonFungible {
                resource_address: id.resource_address,
                local_id: NonFungibleLocalIdId {
                    id_type: String::new(),
                    sbor_hex: String::new(),
                    simple_rep: id.non_fungible_local_id.to_string(),
                },
            },
        }
    }

    fn basic(proof_rule: BasicRequirement) -> CompositeRequirement {
        CompositeRequirement::ProofRule { proof_rule }
    }

    fn require(requirement: Requirement) -> CompositeRequirement {
        basic(BasicRequirement::Require { requirement })
    }

    fn require_resource(resource: ResourceAddress) -> CompositeRequirement {
        require(resource_requirement(resource))
    }

    fn require_nft(resource: ResourceAddress, i: u64) -> CompositeRequirement {
        require(non_fungible_requirement(
            NonFungibleGlobalId::new_unchecked(resource, id(i)),
        ))
    }

    fn amount_of(
        amount: u64,
        resource: ResourceAddress,
    ) -> CompositeRequirement {
        basic(BasicRequirement::AmountOf {
            amount: Decimal192::from(amount),
            resource,
        })
    }

    fn protected(requirement: CompositeRequirement) -> AccessRule {
        AccessRule::Protected {
            access_rule: requirement,
        }
    }

    fn any_of(rules: Vec<CompositeRequirement>) -> CompositeRequirement {
        CompositeRequirement::AnyOf {
            access_rules: rules,
        }
    }

    fn all_of(rules: Vec<CompositeRequirement>) -> CompositeRequirement {
        CompositeRequirement::AllOf {
            access_rules: rules,
        }
    }

    // === What the account can show ===

    fn signer() -> PublicKey {
        PublicKey::sample()
    }

    fn signature_of(key: PublicKey) -> NonFungibleGlobalId {
        (
            ScryptoNonFungibleGlobalId::from_public_key(
                ScryptoPublicKey::from(key),
            ),
            NetworkID::Mainnet,
        )
            .into()
    }

    fn holding(amount: u64) -> BadgeHoldings {
        BadgeHoldings::new([(badge(), Decimal192::from(amount))], [])
    }

    fn holding_nfts(ids: &[u64]) -> BadgeHoldings {
        BadgeHoldings::new(
            [],
            [(nft_badge(), ids.iter().map(|i| id(*i)).collect())],
        )
    }

    fn fungible_proof(
        resource: ResourceAddress,
        amount: u64,
    ) -> ResourceSpecifier {
        ResourceSpecifier::fungible(resource, Decimal192::from(amount))
    }

    fn nft_proof(ids: &[u64]) -> ResourceSpecifier {
        ResourceSpecifier::non_fungible(
            nft_badge(),
            ids.iter().map(|i| id(*i)).collect(),
        )
    }

    fn sut(rule: AccessRule, holdings: &BadgeHoldings) -> SUT {
        SUT::new(&withdraw_rules([(token(), rule)]), [signer()], holdings)
    }

    fn status(
        rule: AccessRule,
        holdings: &BadgeHoldings,
    ) -> RestrictedWithdrawalStatus {
        let sut = sut(rule, holdings);
        assert_eq!(sut.restricted_withdrawals.len(), 1);
        sut.restricted_withdrawals[0].status
    }

    fn withdraw_rules(
        rules: impl IntoIterator<Item = (ResourceAddress, AccessRule)>,
    ) -> WithdrawRules {
        WithdrawRules::new(rules).unwrap()
    }

    // === Tests ===

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn allow_all_is_not_restricted_and_deny_all_is_not_withdrawable() {
        let sut = sut(AccessRule::AllowAll, &holding(1));
        assert_eq!(sut, SUT::default());
        assert!(sut.is_satisfied());

        let sut = self::sut(AccessRule::DenyAll, &holding(1));
        assert!(!sut.is_satisfied());
        assert!(sut.required_badges.is_empty());
        assert_eq!(
            sut.restricted_withdrawals,
            vec![RestrictedWithdrawal::new(
                token(),
                [],
                RestrictedWithdrawalStatus::NotWithdrawable
            )]
        );
    }

    #[test]
    fn a_fungible_badge_is_proven_with_one() {
        let rule = protected(require_resource(badge()));
        let sut = sut(rule.clone(), &holding(5));
        assert!(sut.is_satisfied());
        assert_eq!(sut.required_badges, vec![fungible_proof(badge(), 1)]);
        assert_eq!(
            sut.restricted_withdrawals,
            vec![RestrictedWithdrawal::new(
                token(),
                [badge()],
                RestrictedWithdrawalStatus::Satisfied
            )]
        );

        let sut = self::sut(rule.clone(), &holding(0));
        assert!(!sut.is_satisfied());
        assert!(sut.required_badges.is_empty());
        assert_eq!(
            sut.restricted_withdrawals[0].status,
            RestrictedWithdrawalStatus::MissingBadges
        );
        assert_eq!(
            status(rule, &BadgeHoldings::default()),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn a_non_fungible_badge_is_proven_with_one_of_its_ids() {
        let sut = sut(
            protected(require_resource(nft_badge())),
            &holding_nfts(&[7, 8]),
        );
        assert_eq!(sut.required_badges, vec![nft_proof(&[7])]);
    }

    #[test]
    fn a_specific_non_fungible_must_be_held() {
        let rule = protected(require_nft(nft_badge(), 30));
        // #30# would be beyond the first page of ids of the vault: the Gateway looks it up by
        // its location instead, and puts it in the holdings when the account holds it.
        let sut = sut(rule.clone(), &holding_nfts(&[1, 2, 30]));
        assert_eq!(sut.required_badges, vec![nft_proof(&[30])]);
        assert_eq!(
            status(rule, &holding_nfts(&[1, 2])),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn amount_of_fungible() {
        let rule = protected(amount_of(3, badge()));
        let sut = sut(rule.clone(), &holding(4));
        assert_eq!(sut.required_badges, vec![fungible_proof(badge(), 3)]);
        assert_eq!(
            status(rule, &holding(2)),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn amount_of_non_fungible() {
        let rule = protected(amount_of(2, nft_badge()));
        let sut = sut(rule.clone(), &holding_nfts(&[1, 2, 3]));
        assert_eq!(sut.required_badges, vec![nft_proof(&[1, 2])]);
        assert_eq!(
            status(rule, &holding_nfts(&[1])),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn any_of_takes_the_badge_held() {
        let rule = protected(any_of(vec![
            require_resource(nft_badge()),
            require_resource(badge()),
        ]));
        let sut = sut(rule.clone(), &holding(1));
        assert_eq!(sut.required_badges, vec![fungible_proof(badge(), 1)]);
        assert_eq!(
            sut.restricted_withdrawals[0].badges,
            vec![nft_badge(), badge()]
        );
        assert_eq!(
            status(rule, &BadgeHoldings::default()),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn all_of_needs_every_badge() {
        let rule = protected(all_of(vec![
            require_resource(nft_badge()),
            require_resource(badge()),
        ]));
        assert_eq!(
            status(rule.clone(), &holding(1)),
            RestrictedWithdrawalStatus::MissingBadges
        );
        let both = BadgeHoldings::new(
            [(badge(), Decimal192::one())],
            [(nft_badge(), IndexSet::just(id(1)))],
        );
        let sut = sut(rule, &both);
        assert_eq!(
            sut.required_badges,
            vec![fungible_proof(badge(), 1), nft_proof(&[1])]
        );
    }

    #[test]
    fn basic_all_of_any_of_and_count_of() {
        let list = vec![
            resource_requirement(nft_badge()),
            resource_requirement(badge()),
            resource_requirement(other_badge()),
        ];
        let held = BadgeHoldings::new(
            [
                (badge(), Decimal192::one()),
                (other_badge(), Decimal192::one()),
            ],
            [],
        );

        let count_of_two = protected(basic(BasicRequirement::CountOf {
            count: 2,
            list: list.clone(),
        }));
        assert_eq!(
            sut(count_of_two.clone(), &held).required_badges,
            vec![fungible_proof(badge(), 1), fungible_proof(other_badge(), 1)]
        );
        assert_eq!(
            status(count_of_two, &holding(1)),
            RestrictedWithdrawalStatus::MissingBadges
        );

        let any =
            protected(basic(BasicRequirement::AnyOf { list: list.clone() }));
        assert_eq!(
            sut(any, &held).required_badges,
            vec![fungible_proof(badge(), 1)]
        );

        let all = protected(basic(BasicRequirement::AllOf { list }));
        assert_eq!(
            status(all, &held),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }

    #[test]
    fn the_cheapest_way_is_chosen() {
        // Two badges, or one.
        let rule = protected(any_of(vec![
            all_of(vec![
                require_resource(badge()),
                require_resource(nft_badge()),
            ]),
            require_resource(other_badge()),
        ]));
        let everything = BadgeHoldings::new(
            [
                (badge(), Decimal192::one()),
                (other_badge(), Decimal192::one()),
            ],
            [(nft_badge(), IndexSet::just(id(1)))],
        );
        assert_eq!(
            sut(rule, &everything).required_badges,
            vec![fungible_proof(other_badge(), 1)]
        );
    }

    #[test]
    fn the_badges_proven_for_a_resource_are_preferred_for_the_others() {
        let rules = withdraw_rules([
            (token(), protected(require_resource(other_badge()))),
            (
                other_token(),
                protected(any_of(vec![
                    require_resource(badge()),
                    require_resource(other_badge()),
                ])),
            ),
        ]);
        let holdings = BadgeHoldings::new(
            [
                (badge(), Decimal192::one()),
                (other_badge(), Decimal192::one()),
            ],
            [],
        );
        let sut = SUT::new(&rules, [], &holdings);
        assert!(sut.is_satisfied());
        assert_eq!(sut.required_badges, vec![fungible_proof(other_badge(), 1)]);
        assert_eq!(sut.restricted_withdrawals.len(), 2);
    }

    #[test]
    fn proofs_of_the_same_badge_are_merged() {
        let rules = withdraw_rules([
            (token(), protected(require_resource(badge()))),
            (other_token(), protected(amount_of(2, badge()))),
        ]);
        let sut = SUT::new(&rules, [], &holding(2));
        assert_eq!(sut.required_badges, vec![fungible_proof(badge(), 2)]);
    }

    #[test]
    fn the_signature_of_the_account_needs_no_proof() {
        let rule = protected(require(non_fungible_requirement(signature_of(
            signer(),
        ))));
        let sut = sut(rule, &BadgeHoldings::default());
        assert!(sut.is_satisfied());
        assert!(sut.required_badges.is_empty());
        // A signature is not a badge to show to the person.
        assert!(sut.restricted_withdrawals[0].badges.is_empty());
    }

    #[test]
    fn the_signature_of_another_key_is_not_withdrawable() {
        let rule = protected(require(non_fungible_requirement(signature_of(
            PublicKey::sample_other(),
        ))));
        assert_eq!(
            status(rule, &holding(1)),
            RestrictedWithdrawalStatus::NotWithdrawable
        );
    }

    #[test]
    fn callers_are_not_withdrawable_by_a_transfer() {
        for caller in
            [GLOBAL_CALLER_RESOURCE, PACKAGE_OF_DIRECT_CALLER_RESOURCE]
        {
            let rule = protected(require_resource(implicit(caller)));
            assert_eq!(
                status(rule, &holding(1)),
                RestrictedWithdrawalStatus::NotWithdrawable
            );
        }

        // But another way to satisfy the rule will do.
        let rule = protected(any_of(vec![
            require_resource(implicit(GLOBAL_CALLER_RESOURCE)),
            require_resource(badge()),
        ]));
        let sut = sut(rule.clone(), &holding(1));
        assert_eq!(sut.required_badges, vec![fungible_proof(badge(), 1)]);
        assert_eq!(sut.restricted_withdrawals[0].badges, vec![badge()]);
        // And when it's missing, it's missing.
        assert_eq!(
            status(rule, &holding(0)),
            RestrictedWithdrawalStatus::MissingBadges
        );

        // All of them can never be.
        let rule = protected(all_of(vec![
            require_resource(implicit(GLOBAL_CALLER_RESOURCE)),
            require_resource(badge()),
        ]));
        assert_eq!(
            status(rule, &holding(0)),
            RestrictedWithdrawalStatus::NotWithdrawable
        );
    }

    #[test]
    fn badges_to_look_up() {
        let rules = withdraw_rules([
            (
                token(),
                protected(any_of(vec![
                    require_resource(badge()),
                    amount_of(3, nft_badge()),
                    require_resource(implicit(GLOBAL_CALLER_RESOURCE)),
                    require(non_fungible_requirement(signature_of(signer()))),
                ])),
            ),
            (
                other_token(),
                protected(all_of(vec![
                    require_resource(nft_badge()),
                    require_nft(nft_badge(), 30),
                ])),
            ),
            (other_badge(), AccessRule::AllowAll),
        ]);
        assert_eq!(
            rules.badges_to_look_up(),
            BadgesToLookUp {
                fungibles: IndexSet::just(badge()),
                non_fungibles: IndexMap::from_iter([(nft_badge(), 3)]),
                non_fungible_ids: IndexMap::from_iter([(
                    nft_badge(),
                    IndexSet::just(id(30))
                )]),
            }
        );
        assert!(withdraw_rules([(token(), AccessRule::AllowAll)]).is_empty());
        assert!(withdraw_rules([(token(), AccessRule::DenyAll)])
            .badges_to_look_up()
            .is_empty());
    }

    #[test]
    fn a_signature_resource_is_only_satisfied_by_a_require_of_the_signature() {
        // As in the engine: the implicit signature proofs only count for
        // `Require(NonFungible(signature))`, not for `Require(Resource)` or `AmountOf`.
        let signatures = implicit(SECP256K1_SIGNATURE_RESOURCE);
        assert_eq!(
            status(protected(require_resource(signatures)), &holding(1)),
            RestrictedWithdrawalStatus::NotWithdrawable
        );
        assert_eq!(
            status(protected(amount_of(1, signatures)), &holding(1)),
            RestrictedWithdrawalStatus::NotWithdrawable
        );
        assert_eq!(
            status(
                protected(basic(BasicRequirement::AmountOf {
                    amount: Decimal192::zero(),
                    resource: signatures,
                })),
                &holding(1)
            ),
            RestrictedWithdrawalStatus::NotWithdrawable
        );
    }

    #[test]
    fn amount_of_zero_still_needs_a_proof_of_the_badge() {
        let rule = protected(basic(BasicRequirement::AmountOf {
            amount: Decimal192::zero(),
            resource: badge(),
        }));
        assert_eq!(
            sut(rule.clone(), &holding(3)).required_badges,
            vec![fungible_proof(badge(), 1)]
        );
        assert_eq!(
            status(rule.clone(), &holding(0)),
            RestrictedWithdrawalStatus::MissingBadges
        );
        assert_eq!(
            withdraw_rules([(token(), rule)])
                .badges_to_look_up()
                .fungibles,
            IndexSet::just(badge())
        );

        let nft_rule = protected(basic(BasicRequirement::AmountOf {
            amount: Decimal192::zero(),
            resource: nft_badge(),
        }));
        assert_eq!(
            sut(nft_rule.clone(), &holding_nfts(&[4, 5])).required_badges,
            vec![nft_proof(&[4])]
        );
        assert_eq!(
            withdraw_rules([(token(), nft_rule)])
                .badges_to_look_up()
                .non_fungibles,
            IndexMap::<ResourceAddress, u64>::from_iter([(nft_badge(), 1)])
        );
    }

    #[test]
    fn a_restricted_badge_transferred_too_is_not_withdrawable() {
        // The token needs the badge, the badge needs the other badge, and both are sent: the
        // badge is withdrawn after the proofs are dropped, without its own badge.
        let rules = withdraw_rules([
            (token(), protected(require_resource(badge()))),
            (badge(), protected(require_resource(other_badge()))),
        ]);
        let holdings = BadgeHoldings::new(
            [
                (badge(), Decimal192::one()),
                (other_badge(), Decimal192::one()),
            ],
            [],
        );
        let sut = SUT::new(&rules, [], &holdings);
        assert!(!sut.is_satisfied());
        assert_eq!(
            sut.restricted_withdrawals
                .iter()
                .map(|w| (w.resource, w.status))
                .collect_vec(),
            vec![
                (token(), RestrictedWithdrawalStatus::Satisfied),
                (badge(), RestrictedWithdrawalStatus::NotWithdrawable),
            ]
        );
    }

    #[test]
    fn a_resource_requiring_itself_is_not_withdrawable() {
        let rules =
            withdraw_rules([(badge(), protected(require_resource(badge())))]);
        let sut = SUT::new(&rules, [], &holding(5));
        assert_eq!(
            sut.restricted_withdrawals[0].status,
            RestrictedWithdrawalStatus::NotWithdrawable
        );
    }

    #[test]
    fn rules_that_cannot_be_converted_are_an_error_not_a_panic() {
        let invalid_id = protected(require(Requirement::NonFungible {
            non_fungible: NonFungible {
                resource_address: nft_badge(),
                local_id: NonFungibleLocalIdId {
                    id_type: String::new(),
                    sbor_hex: String::new(),
                    simple_rep: "not an id".to_owned(),
                },
            },
        }));
        assert_eq!(
            WithdrawRules::new([(token(), invalid_id)]),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );

        let count_too_large = protected(basic(BasicRequirement::CountOf {
            count: 300,
            list: vec![resource_requirement(badge())],
        }));
        assert!(WithdrawRules::new([(token(), count_too_large)]).is_err());
    }

    #[test]
    fn a_badge_that_is_not_transferred_is_preferred_to_a_restricted_one_that_is(
    ) {
        // The other token could prove the token, but it is restricted and transferred too.
        let rules = withdraw_rules([
            (
                token(),
                protected(any_of(vec![
                    require_resource(other_token()),
                    require_resource(other_badge()),
                ])),
            ),
            (other_token(), protected(require_resource(badge()))),
        ]);
        let holdings = BadgeHoldings::new(
            [
                (badge(), Decimal192::one()),
                (other_badge(), Decimal192::one()),
            ],
            [(other_token(), IndexSet::just(id(1)))],
        );
        let sut = SUT::new(&rules, [], &holdings);
        assert!(sut.is_satisfied());
        assert_eq!(
            sut.required_badges,
            vec![fungible_proof(other_badge(), 1), fungible_proof(badge(), 1)]
        );
    }

    #[test]
    fn a_fractional_amount_of_proves_the_whole_balance() {
        // Half a badge may not fit its divisibility: prove the balance, which does.
        let rule = protected(basic(BasicRequirement::AmountOf {
            amount: "0.5".parse().unwrap(),
            resource: badge(),
        }));
        assert_eq!(
            sut(rule.clone(), &holding(3)).required_badges,
            vec![fungible_proof(badge(), 3)]
        );
        assert_eq!(
            status(rule, &holding(0)),
            RestrictedWithdrawalStatus::MissingBadges
        );
    }
}
