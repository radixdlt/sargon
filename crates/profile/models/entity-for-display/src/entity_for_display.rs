use crate::prelude::*;

/// A minimal version of an [`Account`] meant for
/// display purposes within wallet
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::From,
)]
#[display("{} | {}", self.display_name(), self.address())]
pub enum EntityForDisplay {
    Account(AccountForDisplay),
    Persona(PersonaForDisplay),
}

impl EntityForDisplay {
    pub fn address(&self) -> AddressOfAccountOrPersona {
        match self {
            Self::Account(account) => account.address.into(),
            Self::Persona(persona) => persona.address.into(),
        }
    }

    pub fn display_name(&self) -> DisplayName {
        match self {
            Self::Account(account) => account.display_name,
            Self::Persona(persona) => persona.display_name,
        }
    }
}

impl HasSampleValues for EntityForDisplay {
    fn sample() -> Self {
        Self::from(AccountForDisplay::sample())
    }

    fn sample_other() -> Self {
        Self::from(PersonaForDisplay::sample())
    }
}

impl Identifiable for EntityForDisplay {
    type ID = AddressOfAccountOrPersona;

    fn id(&self) -> Self::ID {
        self.address()
    }
}

impl IsNetworkAware for EntityForDisplay {
    fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntityForDisplay;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }
}
