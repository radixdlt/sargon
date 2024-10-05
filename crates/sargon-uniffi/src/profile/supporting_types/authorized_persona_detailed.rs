use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{identity_address} | {shared_persona_data}")]
#[serde(rename_all = "camelCase")]
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

impl Identifiable for AuthorizedPersonaDetailed {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address
    }
}

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample() -> AuthorizedPersonaDetailed {
    AuthorizedPersonaDetailed::sample()
}

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample_other(
) -> AuthorizedPersonaDetailed {
    AuthorizedPersonaDetailed::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaDetailed;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_persona_detailed_sample(),
                new_authorized_persona_detailed_sample_other(),
                // duplicates should get removed
                new_authorized_persona_detailed_sample(),
                new_authorized_persona_detailed_sample_other(),
            ])
            .len(),
            2
        );
    }
}
