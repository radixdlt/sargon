use crate::prelude::*;

/// Maps each engine operator to its "at least n of" form, one to one. The
/// result is not simplified, see [`AccessRequirement::simplified`].
impl From<(ScryptoAccessRule, NetworkID)> for AccessRequirement {
    fn from((rule, network_id): (ScryptoAccessRule, NetworkID)) -> Self {
        match rule {
            ScryptoAccessRule::AllowAll => Self::Anyone,
            ScryptoAccessRule::DenyAll => Self::NoOne,
            ScryptoAccessRule::Protected(requirement) => {
                Self::from_composite(requirement, network_id)
            }
        }
    }
}

impl AccessRequirement {
    fn from_composite(
        requirement: ScryptoCompositeRequirement,
        network_id: NetworkID,
    ) -> Self {
        let nested = |requirements: Vec<ScryptoCompositeRequirement>| {
            requirements
                .into_iter()
                .map(|requirement| {
                    Self::from_composite(requirement, network_id)
                })
                .collect::<Vec<_>>()
        };
        match requirement {
            ScryptoCompositeRequirement::BasicRequirement(basic) => {
                Self::from_basic(basic, network_id)
            }
            ScryptoCompositeRequirement::AnyOf(requirements) => {
                Self::any_of(nested(requirements))
            }
            ScryptoCompositeRequirement::AllOf(requirements) => {
                Self::all_of(nested(requirements))
            }
        }
    }

    fn from_basic(
        requirement: ScryptoBasicRequirement,
        network_id: NetworkID,
    ) -> Self {
        let badge = |badge: ScryptoResourceOrNonFungible| {
            if is_proven_by_every_manifest(&badge) {
                Self::Anyone
            } else {
                Self::Badge {
                    badge: (badge, network_id).into(),
                }
            }
        };
        let badges = |badges: Vec<ScryptoResourceOrNonFungible>| {
            badges.into_iter().map(badge).collect::<Vec<_>>()
        };
        match requirement {
            ScryptoBasicRequirement::Require(required) => badge(required),
            ScryptoBasicRequirement::AmountOf(amount, resource) => {
                Self::AmountOf {
                    amount: amount.into(),
                    resource: (resource, network_id).into(),
                }
            }
            ScryptoBasicRequirement::CountOf(count, required) => {
                Self::AtLeast {
                    count: count.into(),
                    requirements: badges(required),
                }
            }
            ScryptoBasicRequirement::AllOf(required) => {
                Self::all_of(badges(required))
            }
            ScryptoBasicRequirement::AnyOf(required) => {
                Self::any_of(badges(required))
            }
        }
    }

    fn any_of(requirements: Vec<Self>) -> Self {
        Self::AtLeast {
            count: 1,
            requirements,
        }
    }

    fn all_of(requirements: Vec<Self>) -> Self {
        Self::AtLeast {
            count: u32::try_from(requirements.len())
                .expect("Requirements of a rule never exceed u32::MAX"),
            requirements,
        }
    }
}

/// Whether `badge` is proven in every method called straight from a
/// manifest, whoever signs it: the engine puts the transaction processor's
/// caller badges in the callee's auth zone, so a rule requiring one passes
/// for anyone calling the method from their own transaction.
fn is_proven_by_every_manifest(badge: &ScryptoResourceOrNonFungible) -> bool {
    let ScryptoResourceOrNonFungible::NonFungible(badge) = badge else {
        return false;
    };
    let transaction_processor = ScryptoBlueprintId::new(
        &TRANSACTION_PROCESSOR_PACKAGE,
        TRANSACTION_PROCESSOR_BLUEPRINT,
    );
    *badge
        == ScryptoNonFungibleGlobalId::package_of_direct_caller_badge(
            TRANSACTION_PROCESSOR_PACKAGE,
        )
        || *badge
            == ScryptoNonFungibleGlobalId::global_caller_badge(
                transaction_processor,
            )
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_common::constants::ACCOUNT_PACKAGE;
    use radix_engine_interface::prelude::{
        global_caller, package_of_direct_caller, require, require_n_of,
    };

    fn key(byte: u8) -> ScryptoNonFungibleGlobalId {
        ScryptoNonFungibleGlobalId::from_public_key(
            radix_common::crypto::Ed25519PublicKey([byte; 32]),
        )
    }

    /// The transaction processor's caller badges are in the auth zone of any
    /// method a manifest calls, so they count as passed for anyone.
    #[test]
    fn transaction_processor_caller_badges_pass_for_anyone() {
        let network_id = NetworkID::Stokenet;
        let transaction_processor = ScryptoBlueprintId::new(
            &TRANSACTION_PROCESSOR_PACKAGE,
            TRANSACTION_PROCESSOR_BLUEPRINT,
        );
        for caller_badge in [
            package_of_direct_caller(TRANSACTION_PROCESSOR_PACKAGE),
            global_caller(transaction_processor),
        ] {
            let alone =
                ScryptoAccessRule::Protected(require(caller_badge.clone()));
            assert_eq!(
                AccessRequirement::from((alone, network_id)).simplified(),
                AccessRequirement::Anyone
            );

            // "Any 2 of a key, the caller badge and another key" is passed by
            // either key alone.
            let in_count = ScryptoAccessRule::Protected(require_n_of(
                2u8,
                vec![key(1).into(), caller_badge, key(2).into()],
            ));
            let requirement =
                AccessRequirement::from((in_count, network_id)).simplified();
            let alone = AuthZone {
                implicit_non_fungibles: [NonFungibleGlobalId::from((
                    key(1),
                    network_id,
                ))]
                .into(),
                proofs: vec![],
            };
            assert!(requirement.is_satisfied_by(&alone));
        }
    }

    /// Another package's caller badge is a badge like any other.
    #[test]
    fn other_caller_badges_are_badges() {
        let rule = ScryptoAccessRule::Protected(require(
            package_of_direct_caller(ACCOUNT_PACKAGE),
        ));
        assert!(matches!(
            AccessRequirement::from((rule, NetworkID::Stokenet)).simplified(),
            AccessRequirement::Badge { .. }
        ));
    }
}
