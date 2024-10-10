use crate::prelude::*;
use sargon::NetworkDefinition as InternalNetworkDefinition;

/// A version of the Radix Network, for a NetworkID with an identifier (name) and display description (display name)
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct NetworkDefinition {
    /// A String identifier (always lowercase) with the name of the Network that MUST match what Gateway returns.
    pub logical_name: String,

    /// The canonical identifier of this network.
    pub id: NetworkID,

    /// A name of the network intended for display purposes only.
    pub display_description: String,
}

impl From<InternalNetworkDefinition> for NetworkDefinition {
    fn from(value: InternalNetworkDefinition) -> Self {
        Self {
            logical_name: value.logical_name,
            id: value.id.into(),
            display_description: value.display_description,
        }
    }
}

impl Into<InternalNetworkDefinition> for NetworkDefinition {
    fn into(self) -> InternalNetworkDefinition {
        InternalNetworkDefinition {
            logical_name: self.logical_name,
            id: self.id.into(),
            display_description: self.display_description,
        }
    }
}

#[uniffi::export]
pub fn new_network_definition_lookup_by_name(
    logical_name: String,
) -> Result<NetworkDefinition> {
    InternalNetworkDefinition::lookup_by_name(&logical_name).map_result()
}

#[uniffi::export]
pub fn new_network_definition_sample() -> NetworkDefinition {
    InternalNetworkDefinition::sample().into()
}

#[uniffi::export]
pub fn new_network_definition_sample_other() -> NetworkDefinition {
    InternalNetworkDefinition::sample_other().into()
}

