use hierarchical_deterministic::bip32::hd_path_component::HDPathValue;
use serde::{Deserialize, Serialize};

use crate::v100::factors::hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnsecuredEntityControl {
    /// The last path component of the SLIP10 derivation path.
    pub entity_index: HDPathValue,

    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl UnsecuredEntityControl {
    pub fn new(
        entity_index: HDPathValue,
        transaction_signing: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self {
            entity_index,
            transaction_signing,
            authentication_signing: Option::None,
        }
    }
}
