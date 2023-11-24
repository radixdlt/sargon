use std::collections::BTreeSet;

use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum SLIP10Curve {
    Curve25519,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCryptoParameters {
    supported_curves: NonEmpty<BTreeSet<SLIP10Curve>>,
}
