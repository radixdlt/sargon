use crate::prelude::*;

/// A key a pre-authorization can be signed with, and the badge its signature
/// proves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreAuthorizationSigningKey {
    pub badge: NonFungibleGlobalId,
    pub factor_instance: HierarchicalDeterministicFactorInstance,
}

pub trait OsPreAuthorizationSigningKeys {
    /// The keys a pre-authorization is signed with for access rules it
    /// doesn't own: the MFA factor instances on the current network.
    ///
    /// Signing picks from these the keys a rule names, and checks of what
    /// the user can authorize alone in a pre-authorization count all of
    /// them, so that the two never disagree.
    fn pre_authorization_signing_keys(
        &self,
    ) -> Result<Vec<PreAuthorizationSigningKey>>;
}

impl OsPreAuthorizationSigningKeys for SargonOS {
    fn pre_authorization_signing_keys(
        &self,
    ) -> Result<Vec<PreAuthorizationSigningKey>> {
        let profile = self.profile()?;
        let network = match profile.current_network() {
            Ok(network) => network,
            Err(CommonError::NoNetworkInProfile { .. }) => {
                return Ok(Vec::new())
            }
            Err(error) => return Err(error),
        };
        network
            .mfa_factor_instances
            .iter()
            .map(|mfa| {
                let badge =
                    mfa.factor_instance.badge.try_non_fungible_global_id()?;
                Ok(PreAuthorizationSigningKey {
                    badge: NonFungibleGlobalId::from((badge, network.id)),
                    factor_instance: mfa
                        .factor_instance
                        .try_as_hd_factor_instances()?,
                })
            })
            .collect()
    }
}
