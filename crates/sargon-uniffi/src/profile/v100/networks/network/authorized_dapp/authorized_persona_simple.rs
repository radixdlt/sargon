use crate::prelude::*;

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
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.description())]
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

impl Identifiable for AuthorizedPersonaSimple {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address
    }
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet() -> AuthorizedPersonaSimple
{
    AuthorizedPersonaSimple::sample_mainnet()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet_other(
) -> AuthorizedPersonaSimple {
    AuthorizedPersonaSimple::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet() -> AuthorizedPersonaSimple
{
    AuthorizedPersonaSimple::sample_stokenet()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet_other(
) -> AuthorizedPersonaSimple {
    AuthorizedPersonaSimple::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaSimple;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_persona_simple_sample_mainnet(),
                new_authorized_persona_simple_sample_mainnet_other(),
                new_authorized_persona_simple_sample_stokenet(),
                new_authorized_persona_simple_sample_stokenet_other(),
                // duplicates should get removed
                new_authorized_persona_simple_sample_mainnet(),
                new_authorized_persona_simple_sample_mainnet_other(),
                new_authorized_persona_simple_sample_stokenet(),
                new_authorized_persona_simple_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
