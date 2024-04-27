use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
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

impl AuthorizedPersonaDetailed {
    pub fn new(
        identity_address: impl Into<IdentityAddress>,
        display_name: impl Into<DisplayName>,
        simple_accounts: impl Into<Option<AccountsForDisplay>>,
        shared_persona_data: PersonaData,
        has_authentication_signing_key: bool,
    ) -> Self {
        Self {
            identity_address: identity_address.into(),
            display_name: display_name.into(),
            simple_accounts: simple_accounts.into(),
            shared_persona_data,
            has_authentication_signing_key,
        }
    }
}

impl HasSampleValues for AuthorizedPersonaDetailed {
    fn sample() -> Self {
        Self::new(
            IdentityAddress::sample(),
            DisplayName::sample(),
            AccountsForDisplay::sample(),
            PersonaData::sample(),
            false,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IdentityAddress::sample_other(),
            DisplayName::sample_other(),
            AccountsForDisplay::sample_other(),
            PersonaData::sample_other(),
            true,
        )
    }
}

impl Identifiable for AuthorizedPersonaDetailed {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address
    }
}

impl IsNetworkAware for AuthorizedPersonaDetailed {
    fn network_id(&self) -> NetworkID {
        self.identity_address.network_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaDetailed;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
