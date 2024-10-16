use crate::prelude::*;
use sargon::AuthorizedDappDetailed as InternalAuthorizedDappDetailed;

#[derive(Clone, PartialEq, Hash, Eq, InternalConversionV2, uniffi::Record)]
pub struct AuthorizedDappDetailed {
    pub network_id: NetworkID,

    pub dapp_definition_address: AccountAddress,

    pub display_name: Option<DisplayName>,

    pub detailed_authorized_personas: Vec<AuthorizedPersonaDetailed>,

    pub preferences: AuthorizedDappPreferences,
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample() -> AuthorizedDappDetailed {
    InternalAuthorizedDappDetailed::sample().into()
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample_other() -> AuthorizedDappDetailed {
    InternalAuthorizedDappDetailed::sample_other().into()
}
