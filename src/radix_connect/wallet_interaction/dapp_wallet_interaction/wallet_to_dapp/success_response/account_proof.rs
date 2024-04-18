use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: DappWalletInteractionAuthProof,
}
