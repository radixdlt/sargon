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
)]
#[display("{display_name} | {address}")]
#[serde(rename_all = "camelCase")]
pub struct BaseEntity<Address> {
    /// The ID of the network this Persona or Account can be used with.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// A globally unique identifier of this Persona or Account, being a human readable
    /// address of an Persona or Account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...nvjdwr`
    ///
    /// No two addresses will ever be the same even for the same factor source
    /// but on different networks, since the public keys controlling the
    /// Persona or Accounts depend on the network id.
    pub address: Address,

    /// An off-ledger display name or description chosen by the user when she
    /// created this Persona or Account.
    pub display_name: DisplayName,

    /// Security state of this Persona or Account, either "securified" or not.
    pub security_state: EntitySecurityState,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Persona or Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    #[serde(default)]
    pub flags: EntityFlags,
}

impl<Address> BaseEntity<Address> {
    pub fn new(
        network_id: impl Into<NetworkID>,
        address: impl Into<Address>,
        display_name: impl Into<DisplayName>,
        security_state: impl Into<EntitySecurityState>,
        flags: impl IntoIterator<Item = EntityFlag>,
    ) -> Self {
        Self {
            network_id: network_id.into(),
            address: address.into(),
            display_name: display_name.into(),
            security_state: security_state.into(),
            flags: flags.into_iter().collect(),
        }
    }
}
