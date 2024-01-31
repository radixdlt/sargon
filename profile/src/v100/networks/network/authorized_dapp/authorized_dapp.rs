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

impl AuthorizedDapp {
    pub fn new(
        network_id: NetworkID,
        dapp_definition_address: DappDefinitionAddress,
        display_name: Option<impl AsRef<str>>,
        references_to_authorized_personas: IdentifiedVecVia<
            AuthorizedPersonaSimple,
        >,
    ) -> Self {
        Self {
            network_id,
            dapp_definition_address,
            display_name: display_name.map(|n| n.as_ref().to_string()),
            references_to_authorized_personas,
        }
    }
}

impl Identifiable for AuthorizedDapp {
    type ID = DappDefinitionAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address.clone()
    }
}

pub type DappDefinitionAddress = AccountAddress;

impl Identifiable for AccountAddress {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl AuthorizedDapp {
    pub fn placeholder_mainnet_dashboard() -> Self {
        Self::new(NetworkID::Mainnet, dapp_definition_address: "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5".parse().expect("Valid Dapp Def Address"), "Dashboard", references_to_authorized_personas: IdentifiedVecVia::from_iter([]))
    }
    pub fn placeholder_mainnet_gumballclub() -> Self {
        todo!()
    }
    pub fn placeholder_stokenet_devconsole() -> Self {
        todo!()
    }
    pub fn placeholder_stokenet_sandbox() -> Self {
        todo!()
    }
}
impl HasPlaceholder for AuthorizedDapp {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_dashboard()
    }
    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_gumballclub()
    }
}
