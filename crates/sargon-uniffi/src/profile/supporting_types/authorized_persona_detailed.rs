use crate::prelude::*;
use sargon::AuthorizedPersonaDetailed as InternalAuthorizedPersonaDetailed;

#[derive(Clone, PartialEq, Hash, Eq, uniffi::Record)]
pub struct AuthorizedPersonaDetailed {
    /// Address that globally and uniquely identifies this Persona.
    pub identity_address: IdentityAddress,

    /// The display name of the Persona, as stored in `Persona`
    pub display_name: DisplayName,

    /// Information of accounts the user has given the Dapp access to,
    /// being the triple `(accountAddress, displayName, appearanceID)`
    pub simple_accounts: Option<AccountsForDisplay>,

    /// The persona data that the user has given the Dapp access to
    pub shared_persona_data: PersonaData,

    /// If this persona has an auth sign key created
    pub has_authentication_signing_key: bool,
}

impl From<InternalAuthorizedPersonaDetailed> for AuthorizedPersonaDetailed {
    fn from(value: InternalAuthorizedPersonaDetailed) -> Self {
        Self {
            identity_address: value.identity_address.into(),
            display_name: value.display_name.into(),
            simple_accounts: value.simple_accounts.map(|v| v.into_type()),
            shared_persona_data: value.shared_persona_data.into(),
            has_authentication_signing_key: value
                .has_authentication_signing_key,
        }
    }
}

impl Into<InternalAuthorizedPersonaDetailed> for AuthorizedPersonaDetailed {
    fn into(self) -> InternalAuthorizedPersonaDetailed {
        InternalAuthorizedPersonaDetailed {
            identity_address: self.identity_address.into(),
            display_name: self.display_name.into(),
            simple_accounts: self
                .simple_accounts
                .map(|v| v.into_internal()),
            shared_persona_data: self.shared_persona_data.into(),
            has_authentication_signing_key: self.has_authentication_signing_key,
        }
    }
}

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample() -> AuthorizedPersonaDetailed {
    InternalAuthorizedPersonaDetailed::sample().into()
}

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample_other(
) -> AuthorizedPersonaDetailed {
    InternalAuthorizedPersonaDetailed::sample_other().into()
}
