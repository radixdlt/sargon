use crate::prelude::*;

/// A key the wallet can sign with for access rules it doesn't own, e.g. a
/// shared account's, and the badge its signature proves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MfaSigningKey {
    pub badge: NonFungibleGlobalId,
    pub factor_instance: HierarchicalDeterministicFactorInstance,
}

pub trait OsMfaSigningKeys {
    /// The keys the wallet signs with for access rules it doesn't own: the
    /// MFA factor instances on the current network.
    ///
    /// Signing picks from these the keys a rule names, and checks of what
    /// the user can authorize alone, e.g. in a pre-authorization, count all
    /// of them, so that the two never disagree.
    fn mfa_signing_keys(&self) -> Result<Vec<MfaSigningKey>>;
}

impl OsMfaSigningKeys for SargonOS {
    fn mfa_signing_keys(&self) -> Result<Vec<MfaSigningKey>> {
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
                Ok(MfaSigningKey {
                    badge: NonFungibleGlobalId::from((badge, network.id)),
                    factor_instance: mfa
                        .factor_instance
                        .try_as_hd_factor_instances()?,
                })
            })
            .collect()
    }
}
