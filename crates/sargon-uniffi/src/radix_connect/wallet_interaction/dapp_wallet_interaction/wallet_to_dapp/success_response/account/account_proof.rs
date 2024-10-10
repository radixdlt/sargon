use crate::prelude::*;
use sargon::WalletToDappInteractionAccountProof as InternalWalletToDappInteractionAccountProof;

#[derive( Clone, PartialEq,  uniffi::Record)]
pub struct WalletToDappInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: WalletToDappInteractionAuthProof,
}

impl From<InternalWalletToDappInteractionAccountProof> for WalletToDappInteractionAccountProof {
    fn from(value: InternalWalletToDappInteractionAccountProof) -> Self {
        Self {
            account_address: value.account_address.into(),
            proof: value.proof.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionAccountProof> for WalletToDappInteractionAccountProof {
    fn into(self) -> InternalWalletToDappInteractionAccountProof {
        InternalWalletToDappInteractionAccountProof {
            account_address: self.account_address.into(),
            proof: self.proof.into(),
        }
    }
}