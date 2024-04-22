use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: DappWalletInteractionAuthProof,
}

impl HasSampleValues for DappWalletInteractionAccountProof {
    fn sample() -> Self {
        Self {
            account_address: AccountAddress::sample(),
            proof: DappWalletInteractionAuthProof::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            account_address: AccountAddress::sample_other(),
            proof: DappWalletInteractionAuthProof::sample_other(),
        }
    }
}