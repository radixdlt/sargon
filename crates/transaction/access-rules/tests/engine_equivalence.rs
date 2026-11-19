//! Checks `AccessRequirement` against the radix engine: for each rule, the
//! engine's authorization (in a `LedgerSimulator`) and `is_satisfied_by` must
//! agree, before and after simplifying.
//!
//! The engine checks a rule protecting the minting of a resource, with the
//! proofs of the transaction.
//!
//! Also checks, without the engine, that random engine rules have the shapes
//! `AccessRequirementForDisplay` documents.

use access_rules::prelude::*;
use addresses::prelude::{NonFungibleGlobalId, ResourceAddress};
use factors::prelude::FactorSource;
use network::prelude::NetworkID;
use numeric::prelude::Decimal192;
use scrypto_test::prelude as st;
use scrypto_test::prelude::FromPublicKey;
use std::collections::HashSet;

const NETWORK: NetworkID = NetworkID::Simulator;

struct Engine {
    ledger: st::DefaultLedgerSimulator,
    account: st::ComponentAddress,
    account_owner: st::NonFungibleGlobalId,
}

/// A proof the transaction creates from the account's vaults.
enum VaultProof {
    Amount(st::ResourceAddress, u64),
    NonFungibles(st::ResourceAddress, Vec<st::NonFungibleLocalId>),
}

impl Engine {
    fn new() -> Self {
        let mut ledger = st::LedgerSimulatorBuilder::new()
            .without_kernel_trace()
            .build();
        let (public_key, _, account) = ledger.new_account(false);
        Self {
            ledger,
            account,
            account_owner: st::NonFungibleGlobalId::from_public_key(public_key),
        }
    }

    /// Creates a resource per rule, whose minting the rule protects.
    fn resources_protected_by(
        &mut self,
        rules: &[st::AccessRule],
    ) -> Vec<st::ResourceAddress> {
        // Batched to stay within the cost limit of a transaction.
        rules
            .chunks(16)
            .flat_map(|chunk| self.create_resources_protected_by(chunk))
            .collect()
    }

    fn create_resources_protected_by(
        &mut self,
        rules: &[st::AccessRule],
    ) -> Vec<st::ResourceAddress> {
        // The role and metadata macros need the Scrypto prelude in scope.
        use scrypto_test::prelude::*;

        let mut builder = ManifestBuilder::new().lock_fee_from_faucet();
        for rule in rules {
            builder = builder.create_fungible_resource(
                OwnerRole::None,
                false,
                0,
                FungibleResourceRoles {
                    mint_roles: mint_roles! {
                        minter => rule.clone();
                        minter_updater => rule!(deny_all);
                    },
                    ..Default::default()
                },
                metadata!(),
                None,
            );
        }
        let resources = self
            .ledger
            .execute_manifest(builder.build(), [])
            .expect_commit_success()
            .new_resource_addresses()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(resources.len(), rules.len());
        resources
    }

    /// Creates a fungible resource with `supply` in the account.
    fn fungible_in_account(&mut self, supply: u64) -> st::ResourceAddress {
        self.ledger
            .create_fungible_resource(supply.into(), 0, self.account)
    }

    /// Creates a non-fungible resource with `#1#`, `#2#` and `#3#` in the
    /// account.
    fn non_fungible_in_account(&mut self) -> st::ResourceAddress {
        self.ledger.create_non_fungible_resource(self.account)
    }

    /// Whether the engine authorizes minting `resource`, with the signatures
    /// of `signers` and the given proofs from the account's vaults.
    fn authorizes(
        &mut self,
        resource: st::ResourceAddress,
        signers: &[st::NonFungibleGlobalId],
        vault_proofs: &[VaultProof],
    ) -> bool {
        let mut builder = st::ManifestBuilder::new().lock_fee_from_faucet();
        let mut initial_proofs = signers.to_vec();
        if !vault_proofs.is_empty() {
            initial_proofs.push(self.account_owner.clone());
        }
        for proof in vault_proofs {
            builder = match proof {
                VaultProof::Amount(resource, amount) => builder
                    .create_proof_from_account_of_amount(
                        self.account,
                        *resource,
                        *amount,
                    ),
                VaultProof::NonFungibles(resource, ids) => builder
                    .create_proof_from_account_of_non_fungibles(
                        self.account,
                        *resource,
                        ids.clone(),
                    ),
            };
        }
        let manifest = builder
            .mint_fungible(resource, 1)
            .try_deposit_entire_worktop_or_abort(self.account, None)
            .build();

        let receipt = self.ledger.execute_manifest(manifest, initial_proofs);
        if receipt.is_commit_success() {
            return true;
        }
        receipt.expect_specific_failure(|error| {
            matches!(
                error,
                st::RuntimeError::SystemModuleError(
                    st::SystemModuleError::AuthError(..)
                )
            )
        });
        false
    }
}

fn sargon_id(id: &st::NonFungibleGlobalId) -> NonFungibleGlobalId {
    (id.clone(), NETWORK).into()
}

fn sargon_resource(resource: st::ResourceAddress) -> ResourceAddress {
    (resource, NETWORK).into()
}

fn signature_badges(count: u64) -> Vec<st::NonFungibleGlobalId> {
    (1..=count)
        .map(|key| {
            st::NonFungibleGlobalId::from_public_key(
                st::Secp256k1PrivateKey::from_u64(key).unwrap().public_key(),
            )
        })
        .collect()
}

/// Asserts that `rule` is evaluated like the engine does, before and after
/// simplifying.
fn assert_evaluated_like_engine(
    rule: &st::AccessRule,
    auth_zone: &AuthZone,
    engine_authorizes: bool,
) {
    let requirement = AccessRequirement::from((rule.clone(), NETWORK));
    assert_eq!(
        requirement.is_satisfied_by(auth_zone),
        engine_authorizes,
        "{rule:?} as {requirement:?} for {auth_zone:?}"
    );
    let simplified = requirement.simplified();
    assert_eq!(
        simplified.is_satisfied_by(auth_zone),
        engine_authorizes,
        "{rule:?} simplified to {simplified:?} for {auth_zone:?}"
    );
}

/// Checks each rule with every subset of `badges` as signers, and returns how
/// many checks the engine denied and authorized.
fn check_with_every_set_of_signers(
    rules: &[st::AccessRule],
    badges: &[st::NonFungibleGlobalId],
) -> [usize; 2] {
    let mut engine = Engine::new();
    let resources = engine.resources_protected_by(rules);
    let mut outcomes = [0; 2];
    for (rule, resource) in rules.iter().zip(resources) {
        for subset in 0..1usize << badges.len() {
            let signers = badges
                .iter()
                .enumerate()
                .filter(|(index, _)| subset & (1 << index) != 0)
                .map(|(_, badge)| badge.clone())
                .collect::<Vec<_>>();
            let auth_zone = AuthZone {
                implicit_non_fungibles: signers.iter().map(sargon_id).collect(),
                proofs: vec![],
            };
            let authorizes = engine.authorizes(resource, &signers, &[]);
            assert_evaluated_like_engine(rule, &auth_zone, authorizes);
            outcomes[authorizes as usize] += 1;
        }
    }
    outcomes
}

/// Every rule of badges without nesting, with up to two of two badges, every
/// count from 0 to one more than the number of badges, and repeated badges.
#[test]
fn every_rule_without_nesting() {
    let badges = signature_badges(2);
    let lists = [
        vec![],
        vec![0],
        vec![1],
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 1],
    ]
    .map(|indices| {
        indices
            .into_iter()
            .map(|index: usize| badges[index].clone().into())
            .collect::<Vec<st::ResourceOrNonFungible>>()
    });

    let mut rules = vec![st::AccessRule::AllowAll, st::AccessRule::DenyAll];
    let mut basic = badges
        .iter()
        .map(|badge| st::BasicRequirement::Require(badge.clone().into()))
        .collect::<Vec<_>>();
    for list in lists {
        basic.extend(
            (0..=list.len() as u8 + 1).map(|count| {
                st::BasicRequirement::CountOf(count, list.clone())
            }),
        );
        basic.push(st::BasicRequirement::AnyOf(list.clone()));
        basic.push(st::BasicRequirement::AllOf(list));
    }
    rules.extend(basic.into_iter().map(|requirement| {
        st::AccessRule::Protected(st::CompositeRequirement::BasicRequirement(
            requirement,
        ))
    }));

    check_with_every_set_of_signers(&rules, &badges);
}

/// Random nested rules of three badges.
#[test]
fn random_nested_rules() {
    let badges = signature_badges(3);
    let mut random = Xorshift(0x5eed_e4f1_0000_0001);
    let rules = (0..32)
        .map(|_| random_rule(&mut random, &badges, 2))
        .collect::<Vec<_>>();

    let outcomes = check_with_every_set_of_signers(&rules, &badges);
    // Guards against random rules that are all trivially allowed or denied.
    assert!(outcomes.iter().all(|count| *count >= 60), "{outcomes:?}");
}

/// The shape of a shield role without threshold factors.
#[test]
fn zero_count_needs_no_proof() {
    let mut engine = Engine::new();
    let [override_badge] = signature_badges(1).try_into().unwrap();
    let rule =
        st::AccessRule::Protected(st::CompositeRequirement::AnyOf(vec![
            st::CompositeRequirement::BasicRequirement(
                st::BasicRequirement::CountOf(0, vec![]),
            ),
            st::CompositeRequirement::BasicRequirement(
                st::BasicRequirement::AnyOf(vec![override_badge.into()]),
            ),
        ]));
    let [resource] = engine
        .resources_protected_by(&[rule.clone()])
        .try_into()
        .unwrap();

    let authorizes = engine.authorizes(resource, &[], &[]);
    assert!(authorizes);
    assert_evaluated_like_engine(&rule, &AuthZone::default(), authorizes);
    assert_eq!(
        AccessRequirement::from((rule, NETWORK)).simplified(),
        AccessRequirement::Anyone
    );
}

#[test]
fn signature_proves_its_non_fungible_but_not_its_resource() {
    let mut engine = Engine::new();
    let [signer] = signature_badges(1).try_into().unwrap();
    let require = |badge: st::ResourceOrNonFungible| {
        st::AccessRule::Protected(st::CompositeRequirement::BasicRequirement(
            st::BasicRequirement::Require(badge),
        ))
    };
    let rules = [
        require(signer.clone().into()),
        require(signer.resource_address().into()),
    ];
    let resources = engine.resources_protected_by(&rules);
    let auth_zone = AuthZone {
        implicit_non_fungibles: HashSet::from([sargon_id(&signer)]),
        proofs: vec![],
    };

    for (rule, resource, expected) in [
        (&rules[0], resources[0], true),
        (&rules[1], resources[1], false),
    ] {
        let authorizes = engine.authorizes(resource, &[signer.clone()], &[]);
        assert_eq!(authorizes, expected, "{rule:?}");
        assert_evaluated_like_engine(rule, &auth_zone, authorizes);
    }
}

#[test]
fn proof_from_a_vault_proves_its_non_fungibles_and_resource() {
    let mut engine = Engine::new();
    let badge_resource = engine.non_fungible_in_account();
    let proven = st::NonFungibleGlobalId::new(
        badge_resource,
        st::NonFungibleLocalId::integer(1),
    );
    let not_proven = st::NonFungibleGlobalId::new(
        badge_resource,
        st::NonFungibleLocalId::integer(2),
    );
    let require = |badge: st::ResourceOrNonFungible| {
        st::AccessRule::Protected(st::CompositeRequirement::BasicRequirement(
            st::BasicRequirement::Require(badge),
        ))
    };
    let rules = [
        require(proven.into()),
        require(badge_resource.into()),
        require(not_proven.into()),
    ];
    let resources = engine.resources_protected_by(&rules);
    let vault_proofs = [VaultProof::NonFungibles(
        badge_resource,
        vec![st::NonFungibleLocalId::integer(1)],
    )];
    let auth_zone = AuthZone {
        implicit_non_fungibles: HashSet::from([sargon_id(
            &engine.account_owner,
        )]),
        proofs: vec![AuthZoneProof {
            resource: sargon_resource(badge_resource),
            amount: Decimal192::one(),
            non_fungible_local_ids: HashSet::from([
                addresses::prelude::NonFungibleLocalId::integer(1),
            ]),
        }],
    };

    for (rule, resource, expected) in [(0, true), (1, true), (2, false)]
        .map(|(index, expected)| (&rules[index], resources[index], expected))
    {
        let authorizes = engine.authorizes(resource, &[], &vault_proofs);
        assert_eq!(authorizes, expected, "{rule:?}");
        assert_evaluated_like_engine(rule, &auth_zone, authorizes);
    }
}

#[test]
fn amount_needs_a_single_proof_of_at_least_the_amount() {
    let mut engine = Engine::new();
    let token = engine.fungible_in_account(10);
    let amount_of = |amount: u64| {
        st::AccessRule::Protected(st::CompositeRequirement::BasicRequirement(
            st::BasicRequirement::AmountOf(amount.into(), token),
        ))
    };
    let rules = [amount_of(5), amount_of(0)];
    let resources = engine.resources_protected_by(&rules);
    let account_owner = sargon_id(&engine.account_owner);
    let auth_zone = |amounts: &[u64]| AuthZone {
        implicit_non_fungibles: if amounts.is_empty() {
            HashSet::new()
        } else {
            HashSet::from([account_owner.clone()])
        },
        proofs: amounts
            .iter()
            .map(|amount| AuthZoneProof {
                resource: sargon_resource(token),
                amount: (*amount).into(),
                non_fungible_local_ids: HashSet::new(),
            })
            .collect(),
    };

    for (rule_index, amounts, expected) in [
        (0, vec![5], true),
        (0, vec![3, 3], false),
        (1, vec![], false),
        (1, vec![1], true),
    ] {
        let vault_proofs = amounts
            .iter()
            .map(|amount| VaultProof::Amount(token, *amount))
            .collect::<Vec<_>>();
        let authorizes =
            engine.authorizes(resources[rule_index], &[], &vault_proofs);
        let rule = &rules[rule_index];
        assert_eq!(authorizes, expected, "{rule:?} with {amounts:?}");
        assert_evaluated_like_engine(rule, &auth_zone(&amounts), authorizes);
    }
}

/// A count above one and below all only ever lists badges, so no engine rule
/// needs rewriting to be shown, and the display has its documented shapes.
#[test]
fn random_rules_have_the_display_shapes() {
    let badges = signature_badges(3);
    let mut random = Xorshift(0x5eed_9a11_0000_0001);
    for _ in 0..5_000 {
        let rule = random_rule(&mut random, &badges, 4);
        let simplified =
            AccessRequirement::from((rule.clone(), NETWORK)).simplified();
        assert_counts_only_list_badges(&simplified, &rule);

        let no_own_factors =
            |_: &NonFungibleGlobalId| -> Option<FactorSource> { None };
        assert_display_shapes(
            &simplified.for_display(&no_own_factors),
            None,
            &rule,
        );
    }
}

fn assert_counts_only_list_badges(
    requirement: &AccessRequirement,
    rule: &st::AccessRule,
) {
    let AccessRequirement::AtLeast {
        count,
        requirements,
    } = requirement
    else {
        return;
    };
    let is_count_of = *count > 1 && (*count as usize) < requirements.len();
    for nested in requirements {
        assert!(
            !is_count_of || matches!(nested, AccessRequirement::Badge { .. }),
            "{rule:?} simplified to a count over {nested:?}"
        );
        assert_counts_only_list_badges(nested, rule);
    }
}

/// `parent` is `None` for a whole role.
fn assert_display_shapes(
    shown: &AccessRequirementForDisplay,
    parent: Option<&AccessRequirementForDisplay>,
    rule: &st::AccessRule,
) {
    use AccessRequirementForDisplay::*;
    match shown {
        Anyone | NoOne => assert!(parent.is_none(), "{rule:?} nests {shown:?}"),
        Badge { .. } | AmountOf { .. } => {}
        CountOf { count, badges } => assert!(
            *count > 1 && (*count as usize) < badges.len(),
            "{rule:?} shows {shown:?}"
        ),
        AnyOf { requirements } | AllOf { requirements } => {
            assert!(requirements.len() >= 2, "{rule:?} shows {shown:?}");
            assert!(
                parent.map(std::mem::discriminant)
                    != Some(std::mem::discriminant(shown)),
                "{rule:?} nests {shown:?} in the same kind"
            );
            for nested in requirements {
                assert_display_shapes(nested, Some(shown), rule);
            }
        }
    }
}

fn random_rule(
    random: &mut Xorshift,
    badges: &[st::NonFungibleGlobalId],
    depth: u32,
) -> st::AccessRule {
    match random.below(10) {
        0 => st::AccessRule::AllowAll,
        1 => st::AccessRule::DenyAll,
        _ => st::AccessRule::Protected(random_composite(random, badges, depth)),
    }
}

fn random_composite(
    random: &mut Xorshift,
    badges: &[st::NonFungibleGlobalId],
    depth: u32,
) -> st::CompositeRequirement {
    if depth == 0 || random.below(2) == 0 {
        return st::CompositeRequirement::BasicRequirement(random_basic(
            random, badges,
        ));
    }
    let nested = (0..random.below(4))
        .map(|_| random_composite(random, badges, depth - 1))
        .collect();
    if random.below(2) == 0 {
        st::CompositeRequirement::AnyOf(nested)
    } else {
        st::CompositeRequirement::AllOf(nested)
    }
}

fn random_basic(
    random: &mut Xorshift,
    badges: &[st::NonFungibleGlobalId],
) -> st::BasicRequirement {
    let badge = |random: &mut Xorshift| -> st::ResourceOrNonFungible {
        badges[random.below(badges.len() as u64) as usize]
            .clone()
            .into()
    };
    let len = random.below(4);
    let list = (0..len).map(|_| badge(random)).collect::<Vec<_>>();
    match random.below(4) {
        0 => st::BasicRequirement::Require(badge(random)),
        1 => st::BasicRequirement::CountOf(random.below(len + 2) as u8, list),
        2 => st::BasicRequirement::AnyOf(list),
        _ => st::BasicRequirement::AllOf(list),
    }
}

struct Xorshift(u64);

impl Xorshift {
    fn below(&mut self, bound: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % bound
    }
}
