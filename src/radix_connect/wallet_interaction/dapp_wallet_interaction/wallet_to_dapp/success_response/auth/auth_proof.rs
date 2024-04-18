use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionAuthProof {
    pub public_key: String,
    pub curve: SLIP10Curve,
    pub signature: String,
}
