use crate::prelude::*;
use std::collections::HashMap;

/// The user's factor sources by the signature badge of each key in the
/// Profile's MFA factor instances, on any network.
///
/// The MFA factor instances are the Profile's registry of keys: the keys in
/// an entity's security state describe a structure the ledger may no longer
/// have, and are not used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnFactorSources(HashMap<ScryptoNonFungibleGlobalId, FactorSource>);

impl OwnFactorSources {
    /// The user's factor source whose key `badge` is the signature badge of,
    /// if any.
    pub fn factor_source_of(
        &self,
        badge: &NonFungibleGlobalId,
    ) -> Option<FactorSource> {
        self.0
            .get(&ScryptoNonFungibleGlobalId::from(badge.clone()))
            .cloned()
    }
}

impl From<&Profile> for OwnFactorSources {
    fn from(profile: &Profile) -> Self {
        Self(
            profile
                .networks
                .iter()
                .flat_map(|network| {
                    network.mfa_factor_instances.iter().collect_vec()
                })
                .filter_map(|mfa| {
                    let instance = mfa.factor_instance;
                    let badge =
                        instance.badge.try_non_fungible_global_id().ok()?;
                    let factor_source = profile
                        .factor_sources
                        .get_id(instance.factor_source_id)?;
                    Some((badge, factor_source.clone()))
                })
                .collect(),
        )
    }
}

pub trait OsOwnFactorSources {
    /// The user's factor sources by badge, see [`OwnFactorSources`].
    fn own_factor_sources(&self) -> Result<OwnFactorSources>;
}

impl OsOwnFactorSources for SargonOS {
    fn own_factor_sources(&self) -> Result<OwnFactorSources> {
        Ok(OwnFactorSources::from(&self.profile()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn finds_factor_sources_of_mfa_keys_only() {
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

        let own = os.own_factor_sources().unwrap();
        let badge_on = |network_id| {
            NonFungibleGlobalId::from((
                mfa.badge.try_non_fungible_global_id().unwrap(),
                network_id,
            ))
        };
        // A key's badge is the same on every network.
        assert_eq!(
            own.factor_source_of(&badge_on(NetworkID::Mainnet)),
            Some(factor_source.clone())
        );
        assert_eq!(
            own.factor_source_of(&badge_on(NetworkID::Stokenet)),
            Some(factor_source)
        );
        assert_eq!(
            own.factor_source_of(&NonFungibleGlobalId::from(
                HierarchicalDeterministicPublicKey::sample_other()
            )),
            None
        );
    }
}
