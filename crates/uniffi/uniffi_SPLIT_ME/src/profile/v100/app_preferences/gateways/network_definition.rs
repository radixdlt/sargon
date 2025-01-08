use crate::prelude::*;
use sargon::NetworkDefinition as InternalNetworkDefinition;

/// A version of the Radix Network, for a NetworkID with an identifier (name) and display description (display name)
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct NetworkDefinition {
    /// A String identifier (always lowercase) with the name of the Network that MUST match what Gateway returns.
    pub logical_name: String,

    /// The canonical identifier of this network.
    pub id: NetworkID,

    /// A name of the network intended for display purposes only.
    pub display_description: String,
}

#[uniffi::export]
pub fn new_network_definition_lookup_by_name(
    logical_name: String,
) -> Result<NetworkDefinition> {
    InternalNetworkDefinition::lookup_by_name(&logical_name).into_result()
}

#[uniffi::export]
pub fn new_network_definition_sample() -> NetworkDefinition {
    InternalNetworkDefinition::sample().into()
}

#[uniffi::export]
pub fn new_network_definition_sample_other() -> NetworkDefinition {
    InternalNetworkDefinition::sample_other().into()
}
