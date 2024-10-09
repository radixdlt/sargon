use crate::prelude::*;
use sargon::AuthorizedDapp as InternalAuthorizedDapp;

/// A connection made between a Radix Dapp and the user.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
     uniffi::Record,
)]
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
    pub references_to_authorized_personas: ReferencesToAuthorizedPersonas,

    /// The preferences the user has configured for this Dapp.
    pub preferences: AuthorizedDappPreferences,
}

pub type DappDefinitionAddress = AccountAddress;

impl From<InternalAuthorizedDapp> for AuthorizedDapp {
    fn from(value: InternalAuthorizedDapp) -> Self {
        Self {
            network_id: value.network_id.into(),
            dapp_definition_address: value.dapp_definition_address.into(),
            display_name: value.display_name,
            references_to_authorized_personas: value.references_to_authorized_personas.into(),
            preferences: value.preferences.into(),
        }
    }
}

impl Into<InternalAuthorizedDapp> for AuthorizedDapp {
    fn into(self) -> InternalAuthorizedDapp {
        InternalAuthorizedDapp {
            network_id: self.network_id.into(),
            dapp_definition_address: self.dapp_definition_address.into(),
            display_name: self.display_name,
            references_to_authorized_personas: self.references_to_authorized_personas.into(),
            preferences: self.preferences.into(),
        }
    }
}


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

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDapp;

    #[test]
    fn samples() {
        assert_eq!(
            new_authorized_dapp_sample_mainnet_dashboard(),
            SUT::sample_mainnet_dashboard()
        );

        assert_eq!(
            new_authorized_dapp_sample_mainnet_gumballclub(),
            SUT::sample_mainnet_gumballclub()
        );

        assert_eq!(
            new_authorized_dapp_sample_stokenet_devconsole(),
            SUT::sample_stokenet_devconsole()
        );

        assert_eq!(
            new_authorized_dapp_sample_stokenet_sandbox(),
            SUT::sample_stokenet_sandbox()
        );
    }
}
