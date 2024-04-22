use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAccountsRequestResponseItem {
    pub accounts: Vec<WalletInteractionWalletAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Exactly32Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proofs: Option<Vec<WalletToDappInteractionAccountProof>>,
}

impl HasSampleValues for WalletToDappInteractionAccountsRequestResponseItem {
    fn sample() -> Self {
        Self {
            accounts: vec![WalletInteractionWalletAccount::sample()],
            challenge: Some(Exactly32Bytes::sample()),
            proofs: Some(vec![WalletToDappInteractionAccountProof::sample()]),
        }
    }

    fn sample_other() -> Self {
        Self {
            accounts: vec![WalletInteractionWalletAccount::sample_other()],
            challenge: Some(Exactly32Bytes::sample_other()),
            proofs: Some(vec![WalletToDappInteractionAccountProof::sample_other()]),
        }
    }
}