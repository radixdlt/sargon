use crate::prelude::*;
use sargon::WalletToDappInteractionAuthProof as InternalWalletToDappInteractionAuthProof;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAuthProof {
    pub public_key: PublicKey,
    pub curve: SLIP10Curve,
    pub signature: Signature,
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_auth_proof_from_signature_with_public_key(
    signature_with_public_key: SignatureWithPublicKey,
) -> WalletToDappInteractionAuthProof {
    InternalWalletToDappInteractionAuthProof::from(
        signature_with_public_key.into_internal(),
    )
    .into()
}
