use crate::prelude::*;

/// A connection made between a Radix Dapp and the user.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedDapp {
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    #[serde(rename = "dAppDefinitionAddress")]
    pub dapp_definition_address: DappDefinitionAddress,

    /// Will be nil if we failed to fetch the Dapp Metadata from On-Network for some reason, and
    /// which is allowed if `isDeveloperMode: true` is set.
    pub display_name: Option<String>,

    pub references_to_authorized_personas:
        IdentifiedVecVia<AuthorizedPersonaSimple>,
}

pub type DappDefinitionAddress = AccountAddress;

impl Identifiable for DappDefinitionAddress {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}
