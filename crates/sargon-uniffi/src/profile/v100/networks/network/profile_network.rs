use crate::prelude::*;

/// [`Accounts`], [`Personas`] and [`AuthorizedDapps`] for some [`ProfileNetwork`]
/// which user has created/interacted with, all on the same [Radix Network][`NetworkDefinition`],
/// identified by `id` ([`NetworkID`]).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.description())]
pub struct ProfileNetwork {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub id: NetworkID,

    /// An ordered set of [`Accounts`]` on this network, which are [`Account`]s
    /// the user has created on this network.
    pub accounts: Accounts,

    /// An ordered set of [`Personas`] on this network, which are [`Persona`]s
    /// the user has created on this network.
    pub personas: Personas,

    /// An ordered set of [`AuthorizedDapps`] on this network, which are
    /// [`AuthorizedDapp`]s that the user has interacted with.
    pub authorized_dapps: AuthorizedDapps,

    /// Configuration related to resources
    pub resource_preferences: ResourcePreferences,
}

use crate::prelude::*;

#[uniffi::export]
pub fn new_profile_network_sample() -> ProfileNetwork {
    ProfileNetwork::sample()
}

#[uniffi::export]
pub fn new_profile_network_sample_other() -> ProfileNetwork {
    ProfileNetwork::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_network_sample(),
                new_profile_network_sample_other(),
                // duplicates should get removed
                new_profile_network_sample(),
                new_profile_network_sample_other(),
            ])
            .len(),
            2
        );
    }
}
