use crate::prelude::*;
use profile_logic::prelude::ProfileNetworkDetailsForAuthorizedDapp as _;
use sargon::ProfileNetwork as InternalProfileNetwork;

decl_vec_samples_for!(ProfileNetworks, ProfileNetwork);

/// [`Accounts`], [`Personas`] and [`AuthorizedDapps`] for some [`ProfileNetwork`]
/// which user has created/interacted with, all on the same [Radix Network][`NetworkDefinition`],
/// identified by `id` ([`NetworkID`]).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ProfileNetwork {
    /// The ID of the network that has been used to generate the `accounts` and `personas`
    /// and on which the `authorizedDapps` have been deployed on.
    pub id: NetworkID,

    /// An ordered set of [`Accounts`]` on this network, which are [`Account`]s
    /// the user has created on this network.
    pub accounts: Vec<Account>,

    /// An ordered set of [`Personas`] on this network, which are [`Persona`]s
    /// the user has created on this network.
    pub personas: Vec<Persona>,

    /// An ordered set of [`AuthorizedDapps`] on this network, which are
    /// [`AuthorizedDapp`]s that the user has interacted with.
    pub authorized_dapps: Vec<AuthorizedDapp>,

    /// Configuration related to resources
    pub resource_preferences: Vec<ResourceAppPreference>,

    /// Pre-derived MFA factor instances
    pub mfa_factor_instances: Vec<MFAFactorInstance>,
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
    profile_network
        .into_internal()
        .details_for_authorized_dapp(&dapp.into_internal())
        .into_result()
}
