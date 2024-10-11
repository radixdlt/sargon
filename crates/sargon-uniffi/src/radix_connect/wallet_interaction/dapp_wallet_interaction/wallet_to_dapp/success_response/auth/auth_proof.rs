use crate::prelude::*;
use sargon::WalletToDappInteractionAuthProof as InternalWalletToDappInteractionAuthProof;

#[derive(Clone, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionAuthProof {
    pub public_key: PublicKey,
    pub curve: SLIP10Curve,
    pub signature: Signature,
}

impl From<InternalWalletToDappInteractionAuthProof>
    for WalletToDappInteractionAuthProof
{
    fn from(value: InternalWalletToDappInteractionAuthProof) -> Self {
        Self {
            public_key: value.public_key.into(),
            curve: value.curve.into(),
            signature: value.signature.into(),
        }
    }
}

impl Into<InternalWalletToDappInteractionAuthProof>
    for WalletToDappInteractionAuthProof
{
    fn into(self) -> InternalWalletToDappInteractionAuthProof {
        InternalWalletToDappInteractionAuthProof {
            public_key: self.public_key.into(),
            curve: self.curve.into(),
            signature: self.signature.into(),
        }
    }
}
