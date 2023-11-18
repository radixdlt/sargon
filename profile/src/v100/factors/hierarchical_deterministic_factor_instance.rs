use radix_engine_common::crypto::PublicKey;
use serde::{Deserialize, Serialize};
use slip10::derivation_path::DerivationPath;

use super::factor_source_id_from_hash::FactorSourceIDFromHash;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: PublicKey,
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicFactorInstance {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        public_key: PublicKey,
        derivation_path: DerivationPath,
    ) -> Self {
        Self {
            factor_source_id,
            public_key,
            derivation_path,
        }
    }
}
