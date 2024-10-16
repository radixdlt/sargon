use crate::prelude::*;
use sargon::AuthorizedDapp as InternalAuthorizedDapp;

/// A connection made between a Radix Dapp and the user.
#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Record)]
pub struct AuthorizedDapp {
    /// The ID of the network the authorized Dapp is on.
    pub network_id: NetworkID,

    /// A `DappDefinitionAddress` is in fact just an alias for
    /// [`AccountAddress`], it is the address of the account
    /// which owns controls the Dapp.
    pub dapp_definition_address: DappDefinitionAddress,

    /// The Display name as sent by the Dapp in any interaction
    /// request (CAP21), e.g. "Radix Dashboard".
    pub display_name: Option<String>,

    /// An order set of `AuthorizedPersonaSimple`s, which is a collection of all
    /// the Personas the user has used to interact with this Dapp, it is called
    /// "references to", since the Personas are not stored in full, that would be
    /// bad duplication of data (which might go stale), instead we refer to the
    /// necessary data by IDs.
    pub references_to_authorized_personas: Vec<AuthorizedPersonaSimple>,

    /// The preferences the user has configured for this Dapp.
    pub preferences: AuthorizedDappPreferences,
}

pub type DappDefinitionAddress = AccountAddress;

json_data_convertible!(AuthorizedDapp);

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_dashboard() -> AuthorizedDapp {
    InternalAuthorizedDapp::sample_mainnet_dashboard().into()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_gumballclub() -> AuthorizedDapp {
    InternalAuthorizedDapp::sample_mainnet_gumballclub().into()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_devconsole() -> AuthorizedDapp {
    InternalAuthorizedDapp::sample_stokenet_devconsole().into()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_sandbox() -> AuthorizedDapp {
    InternalAuthorizedDapp::sample_stokenet_sandbox().into()
}
