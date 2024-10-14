use crate::prelude::*;
use sargon::WalletToDappInteractionAuthProof as InternalWalletToDappInteractionAuthProof;

#[derive(Clone, PartialEq, InternalConversionV2, uniffi::Record)]
pub struct WalletToDappInteractionAuthProof {
    pub public_key: PublicKey,
    pub curve: SLIP10Curve,
    pub signature: Signature,
}