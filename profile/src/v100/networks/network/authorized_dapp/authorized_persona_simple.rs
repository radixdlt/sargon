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
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedPersonaSimple {
    /// The globally unique identifier of a Persona is its address, used
    /// to lookup persona
    pub identity_address: IdentityAddress,

    /// Date of last login for this persona.
    pub last_login: Timestamp,

    /// List of "ongoing accountAddresses" that user given the dApp access to.
    pub shared_accounts: Option<SharedAccounts>,
}

impl AuthorizedPersonaSimple {
    pub fn new(identity_address: IdentityAddress, last_login: Timestamp, shared_accounts: Option<SharedAccounts>) -> Self {
        Self {
            identity_address,
            last_login,
            shared_accounts
        }
    }
}

impl Identifiable for AuthorizedPersonaSimple {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.identity_address.clone()
    }
}

impl AuthorizedPersonaSimple {
    pub fn placeholder_mainnet_satoshi() -> Self {
       Self::new(IdentityAddress::placeholder_mainnet(), now(), Some(SharedAccounts::placeholder()))
    }
    pub fn placeholder_mainnet_batman() -> Self {
        todo!()
    }
    pub fn placeholder_stokenet_hermione() -> Self {
        todo!()
    }
    pub fn placeholder_stokenet_leia() -> Self {
        todo!()
    }
}
impl HasPlaceholder for AuthorizedPersonaSimple {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_satoshi()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_batman()
    }
}
