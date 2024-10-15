use crate::prelude::*;
use sargon::WalletToDappInteractionPersonaProof as InternalWalletToDappInteractionPersonaProof;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionPersonaProof {
    pub identity_address: IdentityAddress,
    pub proof: WalletToDappInteractionAuthProof,
}