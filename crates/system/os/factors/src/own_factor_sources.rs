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
