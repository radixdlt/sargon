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

impl ProfileNetwork {
    pub fn into_internal(&self) -> InternalProfileNetwork {
        self.clone().into()
    }
}

impl From<InternalProfileNetwork> for ProfileNetwork {
    fn from(profile_network: InternalProfileNetwork) -> Self {
        Self {
            id: profile_network.id.into(),
            accounts: profile_network.accounts.into_type(),
            personas: profile_network.personas.into_type(),
            authorized_dapps: profile_network.authorized_dapps.into_type(),
            resource_preferences: profile_network
                .resource_preferences
                .into_type(),
        }
    }
}

impl Into<InternalProfileNetwork> for ProfileNetwork {
    fn into(self) -> InternalProfileNetwork {
        InternalProfileNetwork {
            id: self.id.into(),
            accounts: self.accounts.into_internal(),
            personas: self.personas.into_internal(),
            authorized_dapps: self.authorized_dapps.into_internal(),
            resource_preferences: self
                .resource_preferences
                .into_internal(),
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

#[uniffi::export]
pub fn profile_network_details_for_authorized_dapp(
    profile_network: &ProfileNetwork,
    dapp: &AuthorizedDapp,
) -> Result<AuthorizedDappDetailed> {
    profile_network.into_internal().details_for_authorized_dapp(&dapp.into_internal()).into_result()
}