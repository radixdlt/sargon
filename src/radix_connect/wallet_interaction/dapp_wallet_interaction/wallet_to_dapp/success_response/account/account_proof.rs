use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: WalletToDappInteractionAuthProof,
}

impl HasSampleValues for WalletToDappInteractionAccountProof {
    fn sample() -> Self {
        Self {
            account_address: AccountAddress::sample(),
            proof: WalletToDappInteractionAuthProof::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            account_address: AccountAddress::sample_other(),
            proof: WalletToDappInteractionAuthProof::sample_other(),
        }
    }
}
