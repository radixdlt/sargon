use crate::prelude::*;
use sargon::WalletToDappInteractionAccountProof as InternalWalletToDappInteractionAccountProof;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionAccountProof {
    pub account_address: AccountAddress,
    pub proof: WalletToDappInteractionAuthProof,
}