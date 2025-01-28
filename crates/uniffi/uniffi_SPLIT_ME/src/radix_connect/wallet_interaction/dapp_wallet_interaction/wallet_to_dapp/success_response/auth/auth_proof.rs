use crate::prelude::*;
use sargon::WalletToDappInteractionAuthProof as InternalWalletToDappInteractionAuthProof;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionAuthProof {
    pub public_key: PublicKey,
    pub curve: SLIP10Curve,
    pub signature: Signature,
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_auth_proof_from_intent_signature_of_owner(
    intent_signature_of_owner: IntentSignatureOfOwner,
) -> WalletToDappInteractionAuthProof {
    InternalWalletToDappInteractionAuthProof::from(
        intent_signature_of_owner
            .intent_signature
            .value
            .into_internal(),
    )
    .into()
}
