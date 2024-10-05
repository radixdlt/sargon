use crate::prelude::*;

/// A connection made between a Radix Dapp and the user.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.description())]
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

impl Identifiable for AuthorizedDapp {
    type ID = DappDefinitionAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address
    }
}

impl Identifiable for AccountAddress {
    type ID = Self;

    fn id(&self) -> Self::ID {
        *self
    }
}

json_data_convertible!(AuthorizedDapp);

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_dashboard() -> AuthorizedDapp {
    AuthorizedDapp::sample_mainnet_dashboard()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_mainnet_gumballclub() -> AuthorizedDapp {
    AuthorizedDapp::sample_mainnet_gumballclub()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_devconsole() -> AuthorizedDapp {
    AuthorizedDapp::sample_stokenet_devconsole()
}

#[uniffi::export]
pub fn new_authorized_dapp_sample_stokenet_sandbox() -> AuthorizedDapp {
    AuthorizedDapp::sample_stokenet_sandbox()
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
