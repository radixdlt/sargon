use crate::prelude::*;
use std::collections::HashMap;

pub trait ProfileFactorSourcesByBadge {
    /// The factor source of each MFA factor instance in the Profile, on any
    /// network, by the instance's signature badge.
    ///
    /// Only the MFA factor instances are the Profile's registry of keys: the
    /// keys in an entity's security state describe a structure the ledger may
    /// no longer have, and are not used.
    fn factor_sources_by_badge(
        &self,
    ) -> HashMap<ScryptoNonFungibleGlobalId, FactorSource>;
}

impl ProfileFactorSourcesByBadge for Profile {
    fn factor_sources_by_badge(
        &self,
    ) -> HashMap<ScryptoNonFungibleGlobalId, FactorSource> {
        self.networks
            .iter()
            .flat_map(|network| {
                network.mfa_factor_instances.iter().collect_vec()
            })
            .filter_map(|mfa| {
                let instance = mfa.factor_instance;
                let badge = instance.badge.try_non_fungible_global_id().ok()?;
                let factor_source =
                    self.factor_sources.get_id(instance.factor_source_id)?;
                Some((badge, factor_source.clone()))
            })
            .collect()
    }
}

pub trait OsAccessControllerRuleSetForDisplay {
    /// `rule_set` as shown to the user, see
    /// [`AccessControllerRuleSet::for_display`], with the signature badges of
    /// the Profile's keys resolved to their factor sources.
    fn access_controller_rule_set_for_display(
        &self,
        rule_set: AccessControllerRuleSet,
    ) -> Result<AccessControllerRuleSetForDisplay>;
}

impl OsAccessControllerRuleSetForDisplay for SargonOS {
    fn access_controller_rule_set_for_display(
        &self,
        rule_set: AccessControllerRuleSet,
    ) -> Result<AccessControllerRuleSetForDisplay> {
        let factor_sources = self.profile()?.factor_sources_by_badge();
        Ok(rule_set.for_display(&|badge: &NonFungibleGlobalId| {
            factor_sources
                .get(&ScryptoNonFungibleGlobalId::from(badge.clone()))
                .cloned()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn badge(instance: &FactorInstance) -> NonFungibleGlobalId {
        NonFungibleGlobalId::from((
            instance.badge.try_non_fungible_global_id().unwrap(),
            NetworkID::Mainnet,
        ))
    }

    fn require(badge: NonFungibleGlobalId) -> AccessRequirement {
        AccessRequirement::Badge {
            badge: ResourceOrNonFungible::NonFungible { value: badge },
        }
    }

    #[actix_rt::test]
    async fn rule_set_shows_own_keys_as_factor_sources() {
        let os = SargonOS::boot_test().await.unwrap();
        let mfa = MFAFactorInstance::sample_stokenet_account_securified_idx_0()
            .factor_instance;
        let factor_source = FactorSources::sample_values_all()
            .into_iter()
            .find(|factor_source| factor_source.id() == mfa.factor_source_id)
            .unwrap();
        os.update_profile_with(|profile| {
            if profile.factor_sources.get_id(factor_source.id()).is_none() {
                profile.factor_sources.append(factor_source.clone());
            }
            profile.networks.insert_mfa_factor_instances(
                profile.current_network_id(),
                MFAFactorInstances::just(MFAFactorInstance::new(mfa.clone())),
            );
            Ok(())
        })
        .await
        .unwrap();

        let unknown_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample_other(),
        );
        let rule_set = AccessControllerRuleSet {
            primary_role: require(badge(&mfa)),
            recovery_role: AccessRequirement::AtLeast {
                count: 1,
                requirements: vec![
                    require(badge(&mfa)),
                    require(unknown_key.clone()),
                ],
            },
            confirmation_role: AccessRequirement::NoOne,
        };

        let own = BadgeForDisplay::OwnFactor { factor_source };
        assert_eq!(
            os.access_controller_rule_set_for_display(rule_set).unwrap(),
            AccessControllerRuleSetForDisplay {
                primary_role: AccessRequirementForDisplay::Badge {
                    badge: own.clone()
                },
                recovery_role: AccessRequirementForDisplay::AnyOf {
                    requirements: vec![
                        AccessRequirementForDisplay::Badge { badge: own },
                        AccessRequirementForDisplay::Badge {
                            badge: BadgeForDisplay::UnknownKey {
                                badge: unknown_key
                            }
                        },
                    ],
                },
                confirmation_role: AccessRequirementForDisplay::NoOne,
            }
        );
    }
}
