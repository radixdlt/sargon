use crate::prelude::*;
use sargon::ProfileNetwork as InternalProfileNetwork;

/// [`Accounts`], [`Personas`] and [`AuthorizedDapps`] for some [`ProfileNetwork`]
/// which user has created/interacted with, all on the same [Radix Network][`NetworkDefinition`],
/// identified by `id` ([`NetworkID`]).
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
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

impl From<InternalProfileNetwork> for ProfileNetwork {
    fn from(profile_network: InternalProfileNetwork) -> Self {
        Self {
            id: profile_network.id.into(),
            accounts: profile_network.accounts.into_vec(),
            personas: profile_network.personas.into_vec(),
            authorized_dapps: profile_network.authorized_dapps.into_vec(),
            resource_preferences: profile_network
                .resource_preferences
                .into_vec(),
        }
    }
}

impl Into<InternalProfileNetwork> for ProfileNetwork {
    fn into(self) -> InternalProfileNetwork {
        InternalProfileNetwork {
            id: self.id.into(),
            accounts: self.accounts.into_identified_vec(),
            personas: self.personas.into_identified_vec(),
            authorized_dapps: self.authorized_dapps.into_identified_vec(),
            resource_preferences: self
                .resource_preferences
                .into_identified_vec(),
        }
    }
}

#[uniffi::export]
pub fn new_profile_network_sample() -> ProfileNetwork {
    InternalProfileNetwork::sample().into()
}

#[uniffi::export]
pub fn new_profile_network_sample_other() -> ProfileNetwork {
    InternalProfileNetwork::sample_other().into()
}
