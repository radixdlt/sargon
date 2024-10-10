use crate::prelude::*;
use sargon::AuthorizedPersonaSimple as InternalAuthorizedPersonaSimple;

/// Simple data representation of a Persona the user has shared with a Dapp.
/// Simple meaning "the bare minimum amount of data" that enabled `Sargon` to
/// be able to reconstruct a `AuthorizedPersonaDetailed` value, used to populate
/// views.
///
/// N.B. as of 2024-01-31 of `Sargon` we have not yet implemented the struct
/// `AuthorizedPersonaDetailed` since it is not JSON, but logic, and we have yet
/// to migrate `Sargon` into iOS/Android clients, thus we will defer the work
/// of mapping `AuthorizedPersonaSimple` -> `AuthorizedPersonaDetailed`.
#[derive(
    Clone,
    
    PartialEq,
    Hash,
    Eq,
     uniffi::Record,
)]
pub struct AuthorizedPersonaSimple {
    /// The globally unique identifier of a Persona is its address, used
    /// to lookup persona
    pub identity_address: IdentityAddress,

    /// Date of last login for this persona.
    pub last_login: Timestamp,

    /// List of "ongoing accountAddresses" that user given the dApp access to.
    pub shared_accounts: Option<SharedToDappWithPersonaAccountAddresses>,

    /// ID to PersonaData entries to user has shared with a Dapp.
    pub shared_persona_data: SharedPersonaData,
}

impl From<InternalAuthorizedPersonaSimple> for AuthorizedPersonaSimple {
    fn from(value: InternalAuthorizedPersonaSimple) -> Self {
        Self {
            identity_address: value.identity_address.into(),
            last_login: value.last_login.into(),
            shared_accounts: value.shared_accounts.map(Into::into),
            shared_persona_data: value.shared_persona_data.into(),
        }
    }
}

impl Into<InternalAuthorizedPersonaSimple> for AuthorizedPersonaSimple {
    fn into(self) -> InternalAuthorizedPersonaSimple {
        InternalAuthorizedPersonaSimple {
            identity_address: self.identity_address.into(),
            last_login: self.last_login.into(),
            shared_accounts: self.shared_accounts.map(Into::into),
            shared_persona_data: self.shared_persona_data.into(),
        }
    }
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet() -> AuthorizedPersonaSimple
{
    InternalAuthorizedPersonaSimple::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet_other(
) -> AuthorizedPersonaSimple {
    InternalAuthorizedPersonaSimple::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet() -> AuthorizedPersonaSimple
{
    InternalAuthorizedPersonaSimple::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet_other(
) -> AuthorizedPersonaSimple {
    InternalAuthorizedPersonaSimple::sample_stokenet_other().into()
}

