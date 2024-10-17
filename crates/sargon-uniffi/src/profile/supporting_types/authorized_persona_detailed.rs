use crate::prelude::*;
use sargon::AuthorizedPersonaDetailed as InternalAuthorizedPersonaDetailed;

decl_vec_samples_for!(DetailedAuthorizedPersonas, AuthorizedPersonaDetailed);

#[derive(Clone, PartialEq, Hash, Eq, InternalConversion, uniffi::Record)]
pub struct AuthorizedPersonaDetailed {
    /// Address that globally and uniquely identifies this Persona.
    pub identity_address: IdentityAddress,

    /// The display name of the Persona, as stored in `Persona`
    pub display_name: DisplayName,

    /// Information of accounts the user has given the Dapp access to,
    /// being the triple `(accountAddress, displayName, appearanceID)`
    pub simple_accounts: Option<Vec<AccountForDisplay>>,

    /// The persona data that the user has given the Dapp access to
    pub shared_persona_data: PersonaData,

    /// If this persona has an auth sign key created
    pub has_authentication_signing_key: bool,
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
