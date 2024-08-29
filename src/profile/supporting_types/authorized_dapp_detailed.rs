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
#[display("{dapp_definition_address}")]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedDappDetailed {
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: AccountAddress,

    pub display_name: Option<DisplayName>,

    pub detailed_authorized_personas: DetailedAuthorizedPersonas,

    #[serde(default)]
    pub preferences: AuthorizedDappPreferences,
}

impl AuthorizedDappDetailed {
    pub fn new(
        network_id: impl Into<NetworkID>,
        dapp_definition_address: impl Into<AccountAddress>,
        display_name: impl Into<Option<DisplayName>>,
        detailed_authorized_personas: DetailedAuthorizedPersonas,
        preferences: AuthorizedDappPreferences,
    ) -> Self {
        Self {
            network_id: network_id.into(),
            dapp_definition_address: dapp_definition_address.into(),
            display_name: display_name.into(),
            detailed_authorized_personas,
            preferences,
        }
    }
}

impl HasSampleValues for AuthorizedDappDetailed {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet,
            AccountAddress::sample(),
            DisplayName::sample(),
            DetailedAuthorizedPersonas::sample(),
            AuthorizedDappPreferences::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            NetworkID::Stokenet,
            AccountAddress::sample_other(),
            DisplayName::sample_other(),
            DetailedAuthorizedPersonas::sample_other(),
            AuthorizedDappPreferences::sample_other(),
        )
    }
}

impl Identifiable for AuthorizedDappDetailed {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address
    }
}

impl IsNetworkAware for AuthorizedDappDetailed {
    fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDappDetailed;

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

    #[test]
    fn test_id() {
        assert_eq!(SUT::sample().id(), SUT::sample().dapp_definition_address);
    }
}
