use crate::prelude::*;

/// A minimal version of an [`Persona`] meant for
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
)]
#[display("{display_name} | {address}")]
pub struct PersonaForDisplay {
    pub address: IdentityAddress,

    #[serde(rename = "label")]
    pub display_name: DisplayName,
}

impl PersonaForDisplay {
    pub fn new(
        address: impl Into<IdentityAddress>,
        display_name: impl Into<DisplayName>,
    ) -> Self {
        Self {
            address: address.into(),
            display_name: display_name.into(),
        }
    }
}

impl HasSampleValues for PersonaForDisplay {
    fn sample() -> Self {
        Self::new(IdentityAddress::sample(), DisplayName::sample())
    }

    fn sample_other() -> Self {
        Self::new(IdentityAddress::sample_other(), DisplayName::sample_other())
    }
}

impl Identifiable for PersonaForDisplay {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

impl IsNetworkAware for PersonaForDisplay {
    fn network_id(&self) -> NetworkID {
        self.address.network_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaForDisplay;

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
