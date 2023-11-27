use serde::{Deserialize, Serialize};

use super::factor_source_id_from_hash::FactorSourceIDFromHash;

/// A unique and stable identifier of a FactorSource, e.g. a
/// DeviceFactorSource being a mnemonic securely stored in a
/// device (phone), where the ID of it is the hash of a special
/// key derived near the root of it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "wrongPleaseChangeThisToBeDynamic")]
pub enum FactorSourceID {
    FromHash(FactorSourceIDFromHash),
}
